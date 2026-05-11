//! Declarations related to types in the Calscin AST.

use calsc_utils::hash::HashedString;

use crate::nodes::ASTNode;

/// The AST representation of type. Works on a tree-like structure where nodes can have an "inner" child node that is deeper.
///
/// # Example
/// The type `s32*[35]` will be represented as follows
/// -	`Array(35)`
/// -	-> `Pointer()`
/// - 	-->`Generic(s32)`
pub enum ASTType {
    /// Represents a pointer node. The parameter represents the inner type.
    ///
    /// # Example    
    /// `s32*` would be `Pointer(false, Generic(s32))`
    Pointer(Box<ASTType>),

    /// Represents an array. The first parameter determines the array size and should be an integer literal. The second parameter is the inner type.
    ///
    /// # Examples
    /// `s32[56]` would be `Array(56, Generic(s32))`
    ///
    /// `s32*[56]` would be `Array(56, Pointer(Generic(s32)))`
    Array(Box<ASTNode>, Box<ASTType>),

    /// Represents a generic / normal type. The parameter represents the generic type name as an `HashedString`
    ///
    /// # Example
    /// `s32` would be `Generic(s32)`
    Generic(HashedString),
}
