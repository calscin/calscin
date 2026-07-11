//! The main HIR declarations of Calscin. HIR is used to lower the AST into a guaranteed working form.

#![deny(unsafe_code)]

use std::cell::RefCell;

use calsc_state::session::CompilerSession;
use calsc_typing::ctx::TypeCtx;
use calsc_utils::alloc::arena::ArenaAllocator;

use crate::{buildcache::BuildCache, globalctx::GlobalContext, nodes::HIRNode};

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

thread_local! {
    pub static BUILD_CACHE: RefCell<BuildCache> = RefCell::new(BuildCache::new());
}

#[cfg_attr(feature = "debug", derive(Debug))]
pub struct HIRContext<'session> {
    pub nodes: ArenaAllocator<HIRNode>,
    pub scope: GlobalContext,

    pub type_ctx: TypeCtx,
    pub session: &'session mut CompilerSession,
}

impl<'session> HIRContext<'session> {
    pub fn new(session: &'session mut CompilerSession) -> Self {
        Self {
            nodes: ArenaAllocator::new(),
            scope: GlobalContext::new(),
            type_ctx: TypeCtx::new(),
            session,
        }
    }
}
