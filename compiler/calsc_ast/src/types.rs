//! Declarations related to types in the Calscin AST.

use std::fmt::Display;

use calsc_diagnostics::fmt::fmt_list;

use crate::path::ElementPath;

/// The AST representation of type. Works on a tree-like structure where nodes can have an "inner" child node that is deeper.
///
/// # Example
/// The type `s32*[35]` will be represented as follows
/// -	`Array(35)`
/// -	-> `Pointer()`
/// - 	-->`Generic(s32)`
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(PartialEq, Clone, Hash)]
pub enum ASTType {
    /// Represents a reference node. The parameter represents the inner type.
    ///
    /// # Example
    /// `s32&` would be `Reference(false, Generic(s32))`
    Reference(bool, Box<ASTType>),

    /// Represents a pointer node. The parameter represents the inner type.
    ///
    /// # Example
    /// `s32*` would be `Pointer(false, Generic(s32))`
    Pointer(bool, Box<ASTType>),

    /// Represents an array. The first parameter determines the array size and should be an integer literal. The second parameter is the inner type.
    ///
    /// # Examples
    /// `s32[56]` would be `Array(56, Generic(s32))`
    ///
    /// `s32&[56]` would be `Array(56, Reference(Generic(s32)))`
    Array(Option<usize>, Box<ASTType>),

    /// Represents a generic / normal type. The first parameter represents the generic type name as an `HashedString`.
    /// The second parameter represents the size specifier. The third parameter represents the type parameters
    ///
    ///
    /// # Example
    /// `s32` would be `Generic(s32, None)`
    Generic(ElementPath, Option<usize>, Vec<Box<ASTType>>),

    /// The void type
    Void,
}

/// Represents a simpler stage of AST types that are basically used to generate full `ASTType` trees.
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum SimpleASTType {
    Reference(bool),
    Pointer(bool),
    Array(Option<usize>),
    Generic(ElementPath, Option<usize>, Vec<ASTType>),
}

impl SimpleASTType {
    pub fn is_generic(&self) -> bool {
        match self {
            Self::Generic(_, _, _) => true,
            _ => false,
        }
    }
}

impl Display for ASTType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Generic(name, size_params, type_parameters) => {
                write!(f, "{}", name)?;

                if size_params.is_some() {
                    write!(f, ".{}", size_params.as_ref().unwrap())?;
                }

                if !type_parameters.is_empty() {
                    write!(f, "<{}>", fmt_list(&type_parameters))?;
                }

                Ok(())
            }

            Self::Array(size, inner) => {
                write!(f, "{}[", inner)?;

                if size.is_some() {
                    write!(f, "{}", size.unwrap())?;
                }

                write!(f, "]")
            }

            Self::Reference(mutable, inner) => {
                write!(f, "{}&", inner)?;

                if *mutable {
                    write!(f, " mut")?;
                }

                Ok(())
            }

            Self::Pointer(mutable, inner) => {
                write!(f, "{}*", inner)?;

                if *mutable {
                    write!(f, " mut")?;
                }

                Ok(())
            }

            Self::Void => write!(f, "void"),
        }
    }
}
