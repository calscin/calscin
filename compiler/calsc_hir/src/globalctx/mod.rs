//! Definitions for the HIR global context
//! The global context holds everything static such as:
//! - Types ([`BaseTypeInstance`][`calsc_typing::base::instance::BaseTypeInstance`])
//! - Functions (signatures, local contexts, declaration references)

pub mod key;
pub mod vals;
