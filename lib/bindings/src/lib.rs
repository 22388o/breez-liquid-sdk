use std::sync::Arc;

use anyhow::Result;
use breez_liquid_sdk::{error::*, model::*, sdk::LiquidSdk};

pub fn connect(req: ConnectRequest) -> Result<Arc<BindingLiquidSdk>, LiquidSdkError> {
    let sdk = LiquidSdk::connect(req)?;
    Ok(Arc::from(BindingLiquidSdk { sdk }))
}

pub struct BindingLiquidSdk {
    sdk: Arc<LiquidSdk>,
}

impl BindingLiquidSdk {
    pub fn add_event_listener(&self, listener: Box<dyn EventListener>) -> LiquidSdkResult<String> {
        self.sdk.add_event_listener(listener)
    }

    pub fn remove_event_listener(&self, id: String) -> LiquidSdkResult<()> {
        self.sdk.remove_event_listener(id)
    }

    pub fn get_info(&self, req: GetInfoRequest) -> Result<GetInfoResponse, LiquidSdkError> {
        self.sdk.get_info(req).map_err(Into::into)
    }

    pub fn prepare_send_payment(
        &self,
        req: PrepareSendRequest,
    ) -> Result<PrepareSendResponse, PaymentError> {
        self.sdk.prepare_send_payment(&req)
    }

    pub fn send_payment(
        &self,
        req: PrepareSendResponse,
    ) -> Result<SendPaymentResponse, PaymentError> {
        self.sdk.send_payment(&req)
    }

    pub fn prepare_receive_payment(
        &self,
        req: PrepareReceiveRequest,
    ) -> Result<PrepareReceiveResponse, PaymentError> {
        self.sdk.prepare_receive_payment(&req)
    }

    pub fn receive_payment(
        &self,
        req: PrepareReceiveResponse,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        self.sdk.receive_payment(&req)
    }

    pub fn list_payments(&self) -> Result<Vec<Payment>, PaymentError> {
        self.sdk.list_payments()
    }

    pub fn sync(&self) -> LiquidSdkResult<()> {
        self.sdk.sync().map_err(Into::into)
    }

    pub fn empty_wallet_cache(&self) -> LiquidSdkResult<()> {
        self.sdk.empty_wallet_cache().map_err(Into::into)
    }

    pub fn backup(&self) -> LiquidSdkResult<()> {
        self.sdk.backup().map_err(Into::into)
    }

    pub fn restore(&self, req: RestoreRequest) -> LiquidSdkResult<()> {
        self.sdk.restore(req).map_err(Into::into)
    }
}

uniffi::include_scaffolding!("breez_liquid_sdk");
