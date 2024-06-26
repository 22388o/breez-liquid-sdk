use std::{str::FromStr, sync::Arc};

use anyhow::{anyhow, Result};
use boltz_client::swaps::boltz::RevSwapStates;
use boltz_client::swaps::boltzv2;
use log::{debug, error, info, warn};
use tokio::sync::broadcast;

use crate::model::PaymentState::{Complete, Created, Failed, Pending, TimedOut};
use crate::model::{Config, PaymentTxData, PaymentType, ReceiveSwap};
use crate::{ensure_sdk, utils};
use crate::{
    error::PaymentError, model::PaymentState, persist::Persister, swapper::Swapper,
    wallet::OnchainWallet,
};

/// The minimum acceptable fee rate when claiming using zero-conf
pub const DEFAULT_ZERO_CONF_MIN_FEE_RATE_TESTNET: f32 = 0.1;
pub const DEFAULT_ZERO_CONF_MIN_FEE_RATE_MAINNET: f32 = 0.01;
/// The maximum acceptable amount in satoshi when claiming using zero-conf
pub const DEFAULT_ZERO_CONF_MAX_SAT: u64 = 100_000;

pub(crate) struct ReceiveSwapStateHandler {
    config: Config,
    onchain_wallet: Arc<dyn OnchainWallet>,
    persister: Arc<Persister>,
    swapper: Arc<dyn Swapper>,
    subscription_notifier: broadcast::Sender<String>,
}

impl ReceiveSwapStateHandler {
    pub(crate) fn new(
        config: Config,
        onchain_wallet: Arc<dyn OnchainWallet>,
        persister: Arc<Persister>,
        swapper: Arc<dyn Swapper>,
    ) -> Self {
        let (subscription_notifier, _) = broadcast::channel::<String>(30);
        Self {
            config,
            onchain_wallet,
            persister,
            swapper,
            subscription_notifier,
        }
    }

    pub(crate) fn subscribe_payment_updates(&self) -> broadcast::Receiver<String> {
        self.subscription_notifier.subscribe()
    }

    /// Handles status updates from Boltz for Receive swaps
    pub(crate) async fn on_new_status(&self, update: &boltzv2::Update) -> Result<()> {
        let id = update.id();
        let swap_state = update.status();

        let receive_swap = self
            .persister
            .fetch_receive_swap(id)?
            .ok_or(anyhow!("No ongoing Receive Swap found for ID {id}"))?;

        info!("Handling Receive Swap transition to {swap_state:?} for swap {id}");

        match RevSwapStates::from_str(swap_state) {
            Ok(
                RevSwapStates::SwapExpired
                | RevSwapStates::InvoiceExpired
                | RevSwapStates::TransactionFailed
                | RevSwapStates::TransactionRefunded,
            ) => {
                error!("Swap {id} entered into an unrecoverable state: {swap_state:?}");
                self.update_swap_info(id, Failed, None, None).await?;
                Ok(())
            }
            // The lockup tx is in the mempool and we accept 0-conf => try to claim
            // Execute 0-conf preconditions check
            Ok(RevSwapStates::TransactionMempool) => {
                let boltzv2::Update::TransactionMempool { transaction, .. } = update else {
                    return Err(anyhow!("Unexpected payload from Boltz status stream"));
                };

                let lockup_tx_id = &transaction.id;
                self.update_swap_info(id, Pending, None, Some(lockup_tx_id))
                    .await?;

                if let Some(claim_tx_id) = receive_swap.claim_tx_id {
                    return Err(anyhow!(
                        "Claim tx for Receive Swap {id} was already broadcast: txid {claim_tx_id}"
                    ));
                }

                let lockup_tx = utils::deserialize_tx_hex(&transaction.hex)?;

                // If the amount is greater than the zero-conf limit
                let max_amount_sat = self.config.zero_conf_max_amount_sat();
                let receiver_amount_sat = receive_swap.receiver_amount_sat;
                if receiver_amount_sat > max_amount_sat {
                    warn!("[Receive Swap {id}] Amount is too high to claim with zero-conf ({receiver_amount_sat} sat > {max_amount_sat} sat). Waiting for confirmation...");
                    return Ok(());
                }

                debug!("[Receive Swap {id}] Amount is within valid range for zero-conf ({receiver_amount_sat} < {max_amount_sat} sat)");

                // If the transaction has RBF, see https://github.com/bitcoin/bips/blob/master/bip-0125.mediawiki
                // TODO: Check for inherent RBF by ensuring all tx ancestors are confirmed
                let rbf_explicit = lockup_tx.input.iter().any(|input| input.sequence.is_rbf());
                // let rbf_inherent = lockup_tx_history.height < 0;

                if rbf_explicit {
                    warn!("[Receive Swap {id}] Lockup transaction signals RBF. Waiting for confirmation...");
                    return Ok(());
                }

                debug!("[Receive Swap {id}] Lockup tx does not signal RBF. Proceeding...");

                // If the fees are higher than our estimated value
                let tx_fees: u64 = lockup_tx.all_fees().values().sum();
                let min_fee_rate = self.config.zero_conf_min_fee_rate;
                let lower_bound_estimated_fees = lockup_tx.vsize() as f32 * min_fee_rate * 0.8;

                if lower_bound_estimated_fees > tx_fees as f32 {
                    warn!("[Receive Swap {id}] Lockup tx fees are too low: Expected at least {lower_bound_estimated_fees} sat, got {tx_fees} sat. Waiting for confirmation...");
                    return Ok(());
                }

                debug!("[Receive Swap {id}] Lockup tx fees are within acceptable range ({tx_fees} > {lower_bound_estimated_fees} sat). Proceeding with claim.");

                match self.claim(&receive_swap).await {
                    Ok(_) => {}
                    Err(err) => match err {
                        PaymentError::AlreadyClaimed => {
                            warn!("Funds already claimed for Receive Swap {id}")
                        }
                        _ => error!("Claim for Receive Swap {id} failed: {err}"),
                    },
                }

                Ok(())
            }
            Ok(RevSwapStates::TransactionConfirmed) => {
                // TODO: We need to ensure that the lockup tx is actually confirmed
                // if lockup_tx_history.height <= 0 {
                //     return Err(anyhow!("Tx state mismatch: Lockup transaction was marked as confirmed by the swapper, but isn't."));
                // }

                match receive_swap.claim_tx_id {
                    Some(claim_tx_id) => {
                        warn!("Claim tx for Receive Swap {id} was already broadcast: txid {claim_tx_id}")
                    }
                    None => {
                        self.update_swap_info(&receive_swap.id, Pending, None, None)
                            .await?;
                        match self.claim(&receive_swap).await {
                            Ok(_) => {}
                            Err(err) => match err {
                                PaymentError::AlreadyClaimed => {
                                    warn!("Funds already claimed for Receive Swap {id}")
                                }
                                _ => error!("Claim for Receive Swap {id} failed: {err}"),
                            },
                        }
                    }
                }
                Ok(())
            }

            Ok(_) => {
                debug!("Unhandled state for Receive Swap {id}: {swap_state}");
                Ok(())
            }

            _ => Err(anyhow!(
                "Invalid RevSwapState for Receive Swap {id}: {swap_state}"
            )),
        }
    }

    /// Transitions a Receive swap to a new state
    pub(crate) async fn update_swap_info(
        &self,
        swap_id: &str,
        to_state: PaymentState,
        claim_tx_id: Option<&str>,
        lockup_tx_id: Option<&str>,
    ) -> Result<(), PaymentError> {
        info!(
            "Transitioning Receive swap {swap_id} to {to_state:?} (claim_tx_id = {claim_tx_id:?}, lockup_tx_id = {lockup_tx_id:?})"
        );

        let swap = self
            .persister
            .fetch_receive_swap(swap_id)
            .map_err(|_| PaymentError::PersistError)?
            .ok_or(PaymentError::Generic {
                err: format!("Receive Swap not found {swap_id}"),
            })?;
        let payment_id = claim_tx_id
            .or(lockup_tx_id)
            .map(|id| id.to_string())
            .or(swap.claim_tx_id);

        Self::validate_state_transition(swap.state, to_state)?;
        self.persister.try_handle_receive_swap_update(
            swap_id,
            to_state,
            claim_tx_id,
            lockup_tx_id,
        )?;

        if let Some(payment_id) = payment_id {
            let _ = self.subscription_notifier.send(payment_id);
        }
        Ok(())
    }

    async fn claim(&self, ongoing_receive_swap: &ReceiveSwap) -> Result<(), PaymentError> {
        ensure_sdk!(
            ongoing_receive_swap.claim_tx_id.is_none(),
            PaymentError::AlreadyClaimed
        );
        let swap_id = &ongoing_receive_swap.id;
        let claim_address = self.onchain_wallet.next_unused_address().await?.to_string();
        let claim_tx_id = self
            .swapper
            .claim_receive_swap(ongoing_receive_swap, claim_address)?;

        // We insert a pseudo-claim-tx in case LWK fails to pick up the new mempool tx for a while
        // This makes the tx known to the SDK (get_info, list_payments) instantly
        self.persister.insert_or_update_payment(PaymentTxData {
            tx_id: claim_tx_id.clone(),
            timestamp: None,
            amount_sat: ongoing_receive_swap.receiver_amount_sat,
            fees_sat: 0,
            payment_type: PaymentType::Receive,
            is_confirmed: false,
        })?;

        self.update_swap_info(swap_id, Pending, Some(&claim_tx_id), None)
            .await?;

        Ok(())
    }

    fn validate_state_transition(
        from_state: PaymentState,
        to_state: PaymentState,
    ) -> Result<(), PaymentError> {
        match (from_state, to_state) {
            (_, Created) => Err(PaymentError::Generic {
                err: "Cannot transition to Created state".to_string(),
            }),

            (Created | Pending, Pending) => Ok(()),
            (Complete | Failed | TimedOut, Pending) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to Pending state"),
            }),

            (Created | Pending, Complete) => Ok(()),
            (Complete | Failed | TimedOut, Complete) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to Complete state"),
            }),

            (Created, TimedOut) => Ok(()),
            (_, TimedOut) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to TimedOut state"),
            }),

            (_, Failed) => Ok(()),
        }
    }
}
