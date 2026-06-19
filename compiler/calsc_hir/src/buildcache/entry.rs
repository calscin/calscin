//! Entries inside of the build cache.

use calsc_ast::ASTContext;

use crate::HIRContext;

/// Represents an entry inside of the build cache.
pub struct BuildCacheEntry {
    pub ast_ctx: ASTContext,
    pub hir_ctx: HIRContext,
}

impl BuildCacheEntry {
    pub fn new(ast_ctx: ASTContext) -> Self {
        Self {
            ast_ctx,
            hir_ctx: HIRContext::new(),
        }
    }
}
