//! Declarations related to types in the Calscin AST.

use calsc_utils::hash::HashedString;

use crate::nodes::ASTNode;

pub enum ASTType {
    Pointer(bool, Box<ASTType>),
    Array(Box<ASTNode>, Box<ASTType>),
    Generic(HashedString),
}
