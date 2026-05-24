//! The main HIR declarations of Calscin. HIR is used to lower the AST into a guaranteed working form.

use std::cell::RefCell;

use calsc_utils::alloc::arena::ArenaAllocator;

use crate::{globalctx::GlobalContext, nodes::HIRNode, refs::HIRArenaReference};

pub mod funcs;
pub mod globalctx;
pub mod localctx;
pub mod nodes;
pub mod prelude;
pub mod refs;

thread_local! {
    pub static HIR_CONTEXT: RefCell<HIRContext> = RefCell::new(HIRContext::new());
}

#[cfg_attr(feature = "debug", derive(Debug))]
pub struct HIRContext {
    pub nodes: ArenaAllocator<HIRNode, HIRArenaReference>,
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
