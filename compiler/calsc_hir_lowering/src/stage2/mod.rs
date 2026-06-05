//! Declarations for the second stage of the HIR lowering. The stage 2 has one responsibility:
//! - Parse functions bodies
//!
//! The stage 2 should propagate the function body implementations

pub mod funcs;
pub mod values;
pub mod vars;
