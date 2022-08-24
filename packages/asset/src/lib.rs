// #![warn(missing_docs)]
#![doc(html_logo_url = "../../../uml/logo.jpg")]
//! # Apollo Asset
//!
//! ## Description
//!
//! Apollo DAO offers various strategies to maximize yield across farming products.
//!
//! We need a project that defines tokens struct that the platform will work with.
//!
//! ## Objectives
//!
//! The main goal of the **apollo asset** is to:
//!   - Define AssetInfo and functions
//!
//! ## Use Cases
//!
//! ## Scenarios
//!
//! **See module docs**

/// Asset definition and Asset traits and impl
pub mod asset;
/// Error Handling
pub mod error;

/// Pair Asset
pub mod pair;

/// Query Resolver
pub mod querier;

#[cfg(test)]
/// Unit Tests
mod tests;
