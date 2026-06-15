use calsc_utils::alloc::arena::ArenaAllocatorKey;

use crate::nodes::ASTNode;

pub type ASTArenaReference = ArenaAllocatorKey<ASTNode>;
