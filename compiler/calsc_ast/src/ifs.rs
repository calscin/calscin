use crate::refs::ASTArenaReference;

#[derive(PartialEq, Debug, Clone)]
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
