use cosmwasm_std::{
    ConversionOverflowError, Decimal256RangeExceeded, DecimalRangeExceeded, StdError,
};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Corrupted data found. 8 byte expected.")]
    CorruptedData {},

    #[error("{0}")]
    ConversionOverflowError(#[from] ConversionOverflowError),

    #[error("{0}")]
    DecimalRangeExceeded(#[from] DecimalRangeExceeded),

    #[error("{0}")]
    Decimal256RangeExceeded(#[from] Decimal256RangeExceeded),

    #[error("Failed to initialize strategy token.")]
    FailedToInitializeStrategyToken,

    #[error("Unexpected - should not be called on success")]
    UnExpected,

    #[error("Unknown reply operation")]
    UnknownReply,

    #[error("Distribution schedule is empty")]
    EmptyDistributionSchedule,

    #[error("Invalid date ranges in distribution schedule")]
    InvalidDistributionScheduleRanges,

    #[error("Distribution schedule contains gaps or overlaps")]
    OverlappedDistributionRanges


}
