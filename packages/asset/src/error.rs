use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Native token balance mismatch between the argument and the transferred")]
    TransferBalanceError {},

    #[error("Cannot convert an non-native token to Addr.")]
    NonNativeTokenConversion {},
}
