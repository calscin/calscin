use crate::refs::HIRArenaReference;

/// Represents a branch on an if statement.
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(PartialEq, Clone)]
pub enum IfStatementBranch {
    If {
        condition: HIRArenaReference,
        body: Vec<HIRArenaReference>,
    },

    IfElse {
        condition: HIRArenaReference,
        body: Vec<HIRArenaReference>,
    },

    Else {
        body: Vec<HIRArenaReference>,
    },
}
