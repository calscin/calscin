use calsc_utils::alloc::arena::ArenaAllocatorReference;

#[derive(PartialEq, Debug, Clone)]
pub enum IfStatementBranch {
    If {
        condition: ArenaAllocatorReference,
        body: Vec<ArenaAllocatorReference>,
    },

    IfElse {
        condition: ArenaAllocatorReference,
        body: Vec<ArenaAllocatorReference>,
    },

    Else {
        body: Vec<ArenaAllocatorReference>,
    },
}
