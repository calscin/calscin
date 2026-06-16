//! The visibility in Calscin. Represents the visibility of an element inside of the HIR tree for example.
//! Visibility in Calscin has three levels:
//!
//! - **Public**: The element is accessible by anyone
//! - **Protected**: The element is only accessible inside of the package
//! - **Private**: The element is only accessible inside of the module

use crate::path::ModulePath;

/// The visibility enum
#[derive(Clone, Debug)]
pub enum Visibility {
    /// The element is accessible by anyone
    Public,

    /// The element is only accessible inside of the package
    Protected(ModulePath),

    /// The element is only accessible inside of the module
    Private(ModulePath),

    /// The element can be access but not copied
    Uncopiable,
}

impl Visibility {
    /// Can the element be viewed / accessed by the given module path.
    pub fn can_view(&self, path: &ModulePath) -> bool {
        match self {
            Self::Public => true,
            Self::Uncopiable => true,
            Self::Protected(inner_path) => path.package == inner_path.package,
            Self::Private(inner_path) => {
                path.package == inner_path.package && path.path == inner_path.path
            }
        }
    }

    pub fn can_be_imported(&self) -> bool {
        match self {
            Self::Uncopiable => false,

            _ => true,
        }
    }
}
