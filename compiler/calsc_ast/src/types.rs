//! Declarations related to types in the Calscin AST.

use calsc_utils::hash::HashedString;

/// The AST representation of type. Works on a tree-like structure where nodes can have an "inner" child node that is deeper.
///
/// # Example
/// The type `s32*[35]` will be represented as follows
/// -	`Array(35)`
/// -	-> `Pointer()`
/// - 	-->`Generic(s32)`
#[derive(Debug, PartialEq, Clone)]
pub enum ASTType {
    /// Represents a reference node. The parameter represents the inner type.
    ///
    /// # Example    
    /// `s32&` would be `Reference(false, Generic(s32))`
    Reference(bool, Box<ASTType>),

    /// Represents an array. The first parameter determines the array size and should be an integer literal. The second parameter is the inner type.
    ///
    /// # Examples
    /// `s32[56]` would be `Array(56, Generic(s32))`
    ///
    /// `s32&[56]` would be `Array(56, Reference(Generic(s32)))`
    Array(usize, Box<ASTType>),

    /// Represents a generic / normal type. The first parameter represents the generic type name as an `HashedString`. The second parameter represents the size specifier
    /// The third parameter represents any type parameters
    ///
    ///
    /// # Example
    /// `s32` would be `Generic(s32, None, [])`
    ///
    /// `s.32<test>` would be `Generic(s, 32, [test])`
    Generic(HashedString, Option<usize>, Vec<ASTType>),
}

/// Represents a simpler stage of AST types that are basically used to generate full `ASTType` trees.
pub enum SimpleASTType {
    Reference(bool),
    Array(usize),
    Generic(HashedString, Option<usize>, Vec<ASTType>),
}

impl SimpleASTType {
    pub fn is_generic(&self) -> bool {
        match self {
            Self::Generic(_, _, _) => true,
            _ => false,
        }
    }
}
