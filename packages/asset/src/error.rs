use cosmwasm_std::StdError;
use thiserror::Error;

/// Contract Error Handler
#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    /// Standard Error
    #[error("{0}")]
    Std(#[from] StdError),

    /// Authorization
    #[error("Unauthorized")]
    Unauthorized {},

    /// Transfer
    #[error("Native token balance mismatch between the argument and the transferred")]
    TransferBalanceError {},

    /// Token
    #[error("Cannot convert an non-native token to Addr.")]
    NonNativeTokenConversion {},
}
