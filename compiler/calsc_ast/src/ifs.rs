use calsc_utils::alloc::arena::ArenaHandle;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(PartialEq, Clone)]
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
