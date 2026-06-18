//! Entries inside of the build cache.

use calsc_ast::ASTContext;

use crate::HIRContext;

/// Represents an entry inside of the build cache.
/// Allows for stage tracking.
pub enum BuildCacheEntry {
    /// Represents a build entry that reached AST level.
    AST(ASTContext),

    /// Represents a build entry that reached HIR stage 1 level.
    HIRStage1(ASTContext, HIRContext),

    /// Represents a build entry that reached HIR stage 2 level.
    HIRStage2(ASTContext, HIRContext),
}
