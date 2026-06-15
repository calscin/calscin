use crate::nodes::ASTNode;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(PartialEq, Clone)]
pub enum IfStatementBranch {
    If {
        condition: &'static ASTNode,
        body: Vec<&'static ASTNode>,
    },

    IfElse {
        condition: &'static ASTNode,
        body: Vec<&'static ASTNode>,
    },

    Else {
        body: Vec<&'static ASTNode>,
    },
}
