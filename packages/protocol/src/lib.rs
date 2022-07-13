#![warn(missing_docs)]
#![doc(html_logo_url = "../../../uml/logo.jpg")]
//! # Apollo Protocol
//!
//! ## Description
//!
//! Apollo DAO offers various strategies to maximize yield across farming products.
//!
//! We need a project that defines all the handles/interfaces that the platform will work with.
//!
//! ## Objectives
//!
//! The main goal of the **apollo protocol** is to:
//!   - Define how strategies must behave (implementing the Traits)
//!   - Define how the platform Query the blockchain
//!   - Define how messages interact (consume, produce)
//!
//! ## Use Cases
//!
//! ![Use Cases](../../../uml/usecase-apollo-protocol.drawio.svg)
//!
//! ## Scenarios
//!
//! **See module docs**

/// Interface Adapter Layer
pub mod adaptor;
/// Revenue collector
pub mod collector;
/// Documentation
pub mod docs;
/// Error Handler
pub mod error;
/// Strategies factory
pub mod factory;
/// to be removed
pub mod legacy_strategy;
/// to be removed
pub mod legacy_vault;
/// Price oracle
pub mod oracle;
/// querier
pub mod querier;
/// router
pub mod router;
/// Strategy Trait
pub mod strategy;
/// Strategy Token
pub mod strategy_token;
/// Utils
pub mod utils;
/// Generic vault and strategy specific functions
pub mod vault;

#[cfg(test)]
/// Unit Tests
mod utests;
