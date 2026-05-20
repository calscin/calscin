//! The node declarations for the HIR.

use calsc_utils::pos::FilePosition;

/// Represents the kind of the HIR node. Holds information related to the HIR node directly
pub enum HIRNodeKind {}

/// Represents a full HIR node. Holds the node kind and the start and end positions of it
pub struct HIRNode {
    pub kind: HIRNodeKind,
    pub start: FilePosition,
    pub end: FilePosition,
}

impl HIRNode {
    pub fn new(kind: HIRNodeKind, start: FilePosition, end: FilePosition) -> Self {
        Self { kind, start, end }
    }
}
