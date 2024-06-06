use std::collections::HashMap;

use anyhow::Result;
use boltz_client::swaps::boltzv2::{ChainSwapDetails, CreateChainResponse};
use rusqlite::{named_params, params, Connection, Row};
use serde::{Deserialize, Serialize};

use crate::ensure_sdk;
use crate::error::PaymentError;
use crate::model::*;
use crate::persist::Persister;

impl Persister {
    pub(crate) fn insert_chain_swap(&self, chain_swap: &ChainSwap) -> Result<()> {
        let con = self.get_connection()?;

        let mut stmt = con.prepare(
            "
            INSERT INTO chain_swaps (
                id,
                payment_type,
                address,
                preimage,
                payer_amount_sat,
                receiver_amount_sat,
                create_response_json,
                claim_private_key,
                refund_private_key,
                lockup_tx_id,
                refund_tx_id,
                created_at,
                state
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )?;
        _ = stmt.execute((
            &chain_swap.id,
            &chain_swap.payment_type,
            &chain_swap.address,
            &chain_swap.preimage,
            &chain_swap.payer_amount_sat,
            &chain_swap.receiver_amount_sat,
            &chain_swap.create_response_json,
            &chain_swap.claim_private_key,
            &chain_swap.refund_private_key,
            &chain_swap.lockup_tx_id,
            &chain_swap.refund_tx_id,
            &chain_swap.created_at,
            &chain_swap.state,
        ))?;

        Ok(())
    }

    fn list_chain_swaps_query(where_clauses: Vec<String>) -> String {
        let mut where_clause_str = String::new();
        if !where_clauses.is_empty() {
            where_clause_str = String::from("WHERE ");
            where_clause_str.push_str(where_clauses.join(" AND ").as_str());
        }

        format!(
            "
            SELECT
                id,
                payment_type,
                address,
                preimage,
                payer_amount_sat,
                receiver_amount_sat,
                create_response_json,
                claim_private_key,
                refund_private_key,
                lockup_tx_id,
                refund_tx_id,
                created_at,
                state
            FROM chain_swaps
            {where_clause_str}
            ORDER BY created_at
        "
        )
    }

    pub(crate) fn fetch_chain_swap(&self, id: &str) -> Result<Option<ChainSwap>> {
        let con: Connection = self.get_connection()?;
        let query = Self::list_chain_swaps_query(vec!["id = ?1".to_string()]);
        let res = con.query_row(&query, [id], Self::sql_row_to_chain_swap);

        Ok(res.ok())
    }


    fn sql_row_to_chain_swap(row: &Row) -> rusqlite::Result<ChainSwap> {
        Ok(ChainSwap {
            id: row.get(0)?,
            payment_type: row.get(1)?,
            address: row.get(2)?,
            preimage: row.get(3)?,
            payer_amount_sat: row.get(4)?,
            receiver_amount_sat: row.get(5)?,
            create_response_json: row.get(6)?,
            claim_private_key: row.get(7)?,
            refund_private_key: row.get(8)?,
            lockup_tx_id: row.get(9)?,
            refund_tx_id: row.get(10)?,
            created_at: row.get(11)?,
            state: row.get(12)?,
        })
    }

    pub(crate) fn list_chain_swaps(
        &self,
        con: &Connection,
        where_clauses: Vec<String>,
    ) -> rusqlite::Result<Vec<ChainSwap>> {
        let query = Self::list_chain_swaps_query(where_clauses);
        let chain_swaps = con
            .prepare(&query)?
            .query_map(params![], Self::sql_row_to_chain_swap)?
            .map(|i| i.unwrap())
            .collect();
        Ok(chain_swaps)
    }

    pub(crate) fn list_ongoing_chain_swaps(
        &self,
        con: &Connection,
    ) -> rusqlite::Result<Vec<ChainSwap>> {
        let mut where_clause: Vec<String> = Vec::new();
        where_clause.push(format!(
            "state in ({})",
            [PaymentState::Created, PaymentState::Pending]
                .iter()
                .map(|t| format!("'{}'", *t as i8))
                .collect::<Vec<_>>()
                .join(", ")
        ));

        self.list_chain_swaps(con, where_clause)
    }

    pub(crate) fn list_pending_chain_swaps(&self) -> Result<Vec<ChainSwap>> {
        let con: Connection = self.get_connection()?;
        let query = Self::list_chain_swaps_query(vec!["state = ?1".to_string()]);
        let res = con
            .prepare(&query)?
            .query_map(params![PaymentState::Pending], Self::sql_row_to_chain_swap)?
            .map(|i| i.unwrap())
            .collect();
        Ok(res)
    }

    /// Pending Chain swaps, indexed by refund tx id
    pub(crate) fn list_pending_chain_swaps_by_refund_tx_id(
        &self,
    ) -> Result<HashMap<String, ChainSwap>> {
        let res: HashMap<String, ChainSwap> = self
            .list_pending_chain_swaps()?
            .iter()
            .filter_map(|pending_chain_swap| {
                pending_chain_swap
                    .refund_tx_id
                    .as_ref()
                    .map(|refund_tx_id| (refund_tx_id.clone(), pending_chain_swap.clone()))
            })
            .collect();
        Ok(res)
    }

    pub(crate) fn try_handle_chain_swap_update(
        &self,
        swap_id: &str,
        to_state: PaymentState,
        lockup_tx_id: Option<&str>,
        refund_tx_id: Option<&str>,
    ) -> Result<(), PaymentError> {
        // Do not overwrite lockup_tx_id, refund_tx_id
        let con: Connection = self.get_connection()?;
        con.execute(
            "UPDATE chain_swaps
            SET
                lockup_tx_id =
                    CASE
                        WHEN lockup_tx_id IS NULL THEN :lockup_tx_id
                        ELSE lockup_tx_id
                    END,

                refund_tx_id =
                    CASE
                        WHEN refund_tx_id IS NULL THEN :refund_tx_id
                        ELSE refund_tx_id
                    END,

                state = :state
            WHERE
                id = :id",
            named_params! {
                ":id": swap_id,
                ":lockup_tx_id": lockup_tx_id,
                ":refund_tx_id": refund_tx_id,
                ":state": to_state,
            },
        )
        .map_err(|_| PaymentError::PersistError)?;

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct InternalCreateChainResponse {
    pub(crate) claim_details: ChainSwapDetails,
    pub(crate) lockup_details: ChainSwapDetails,
}
impl InternalCreateChainResponse {
    pub(crate) fn try_convert_from_boltz(
        boltz_create_response: &CreateChainResponse,
        expected_swap_id: &str,
    ) -> Result<InternalCreateChainResponse, PaymentError> {
        // Do not store the CreateResponse fields that are already stored separately
        // Before skipping them, ensure they match the separately stored ones
        ensure_sdk!(
            boltz_create_response.id == expected_swap_id,
            PaymentError::PersistError
        );

        let res = InternalCreateChainResponse {
            claim_details: boltz_create_response.claim_details.clone(),
            lockup_details: boltz_create_response.lockup_details.clone(),
        };
        Ok(res)
    }
}
