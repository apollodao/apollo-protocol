use cosmwasm_std::{ConversionOverflowError, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Cannot set to own account")]
    CannotSetOwnAccount {},

    #[error("Invalid zero amount")]
    InvalidZeroAmount {},

    #[error("Allowance is expired")]
    Expired {},

    #[error("No allowance for this account")]
    NoAllowance {},

    #[error("Minting cannot exceed the cap")]
    CannotExceedCap {},

    #[error("Logo binary data exceeds 5KB limit")]
    LogoTooBig {},

    #[error("Invalid xml preamble for SVG")]
    InvalidXmlPreamble {},

    #[error("Invalid png header")]
    InvalidPngHeader {},

    #[error("Duplicate initial balance addresses")]
    DuplicateInitialBalanceAddresses {},

    // Cw4626 errors
    #[error("Withdrawals disabled")]
    WithdrawalsDisabled {},

    #[error("Deposits disabled")]
    DepositsDisabled {},

    #[error("No adaptor provided")]
    NoAdaptor {},

    #[error("No adaptor init msg provided")]
    NoAdaptorInitMsg {},

    #[error("Unsupported message")]
    UnsupportedMessage {},
}

impl From<ConversionOverflowError> for ContractError {
    fn from(_: ConversionOverflowError) -> Self {
        ContractError::Std(StdError::generic_err("Conversion overflow"))
    }
}

impl From<cw20_base::ContractError> for ContractError {
    fn from(e: cw20_base::ContractError) -> Self {
        match e {
            cw20_base::ContractError::Std(e) => ContractError::Std(e),
            cw20_base::ContractError::Unauthorized {} => ContractError::Unauthorized {},
            cw20_base::ContractError::CannotSetOwnAccount {} => {
                ContractError::CannotSetOwnAccount {}
            }
            cw20_base::ContractError::InvalidZeroAmount {} => ContractError::InvalidZeroAmount {},
            cw20_base::ContractError::Expired {} => ContractError::Expired {},
            cw20_base::ContractError::NoAllowance {} => ContractError::NoAllowance {},
            cw20_base::ContractError::CannotExceedCap {} => ContractError::CannotExceedCap {},
            cw20_base::ContractError::LogoTooBig {} => ContractError::LogoTooBig {},
            cw20_base::ContractError::InvalidXmlPreamble {} => ContractError::InvalidXmlPreamble {},
            cw20_base::ContractError::InvalidPngHeader {} => ContractError::InvalidPngHeader {},
            cw20_base::ContractError::DuplicateInitialBalanceAddresses {} => {
                ContractError::DuplicateInitialBalanceAddresses {}
            }
        }
    }
}
