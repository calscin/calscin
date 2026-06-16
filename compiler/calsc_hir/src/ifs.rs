use calsc_utils::alloc::arena::ArenaHandle;

/// Represents a branch on an if statement.
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub enum IfStatementBranch {
    If {
        condition: ArenaHandle,
        body: Vec<ArenaHandle>,
    },

    IfElse {
        condition: ArenaHandle,
        body: Vec<ArenaHandle>,
    },

    Else {
        body: Vec<ArenaHandle>,
    },
}
