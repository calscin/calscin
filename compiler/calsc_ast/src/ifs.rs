use crate::nodes::ASTNode;

#[derive(PartialEq, Debug)]
pub enum IfStatementBranch {
    If {
        condition: Box<ASTNode>,
        body: Vec<Box<ASTNode>>,
    },

    IfElse {
        condition: Box<ASTNode>,
        body: Vec<Box<ASTNode>>,
    },

    Else {
        body: Vec<Box<ASTNode>>,
    },
}
