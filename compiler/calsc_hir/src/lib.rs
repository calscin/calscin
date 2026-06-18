//! The main HIR declarations of Calscin. HIR is used to lower the AST into a guaranteed working form.

#![deny(unsafe_code)]

use calsc_utils::alloc::arena::ArenaAllocator;

use crate::{globalctx::GlobalContext, nodes::HIRNode};

pub mod buildcache;
pub mod conv;
pub mod file;
pub mod funcs;
pub mod globalctx;
pub mod ifs;
pub mod imports;
pub mod localctx;
pub mod nodes;
pub mod prelude;
pub mod types;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)] // For MIR
pub struct HIRContext {
    pub nodes: ArenaAllocator<HIRNode>,
    pub scope: GlobalContext,
}

impl HIRContext {
    pub fn new() -> Self {
        Self {
            nodes: ArenaAllocator::new(),
            scope: GlobalContext::new(),
        }
    }
}

impl Default for HIRContext {
    fn default() -> Self {
        HIRContext::new()
    }
}
