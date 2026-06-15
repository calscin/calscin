use crate::refs::ASTArenaReference;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(PartialEq, Clone)]
pub enum IfStatementBranch {
    If {
        condition: ASTArenaReference,
        body: Vec<ASTArenaReference>,
    },

    IfElse {
        condition: ASTArenaReference,
        body: Vec<ASTArenaReference>,
    },

    Else {
        body: Vec<ASTArenaReference>,
    },
}
