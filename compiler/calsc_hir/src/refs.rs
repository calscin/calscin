#[cfg(feature = "debug")]
use std::fmt::Debug;

use std::ops::Deref;

use calsc_utils::alloc::arena::ArenaAllocatorKey;

use crate::nodes::HIRNode;

pub type HIRArenaReference = ArenaAllocatorKey<HIRNode>;
