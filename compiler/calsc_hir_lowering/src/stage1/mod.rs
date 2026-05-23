//! Declarations for the first stage of the HIR lowering. The stage 1 has a couple of responsibilities:
//! - Add types to the global scope
//! - Add stage 1 functions to the global scope
//! - Add extern function to the global scope

pub mod funcs;
pub mod types;
