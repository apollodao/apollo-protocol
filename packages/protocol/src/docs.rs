#![warn(missing_docs)]
//! # Scenarios
//!
//! ## 01 - Allow configuration over strategies
//!
//! ### Description
//!
//! As Apollo platform we want to manage different states in strategies that allow us to:
//! - Pause an strategy
//! - Deprecate an strategy
//! - Disable withdrawals in case of exploit to SAFU.
//! - Disable deposits on case of exploit or deprecated strategy.
//!
//! ### Actors
//!
//! - Governance contract
//!
//! ### Preconditions
//!
//! - Contract init.
//!
//! ### Dependencies
//!
//! - Base strategy package.
//!
//! ### Flow **Update Strategy**
//!
//! 1. The **Governance contract** call **handle_update_strategy** with the desire final state (pause, deprecate, disable withdraw,disable deposit)
//!
//! ### Post-Conditions
//!
//! The **State** of the strategy change to the desire state.
