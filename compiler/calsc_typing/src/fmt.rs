use std::fmt::Display;

use calsc_diagnostics::fmt::fmt_list;

use crate::{
    base::{BaseType, instance::BaseTypeInstance, kind::BaseTypeKind},
    tree::Type,
};

impl Display for BaseTypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Boolean => "bool",
            Self::Char => "char",
            Self::String => "str",
            Self::Floating { signed } => {
                if *signed {
                    "f"
                } else {
                    "uf"
                }
            }

            Self::Integer { signed } => {
                if *signed {
                    "s"
                } else {
                    "u"
                }
            }

            Self::Struct(container) => &container.name,
        };

        write!(f, "{}", s)
    }
}

impl Display for BaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)?;

        if !self.type_params.is_empty() {
            write!(f, "<{}", self.type_params_iter[0])?;

            for i in 1..self.type_params_iter.len() {
                write!(f, ", {}", self.type_params_iter[i])?;
            }

            write!(f, ">")?;
        }

        Ok(())
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Array { size, inner } => {
                write!(f, "{}[", inner)?;

                if size.is_some() {
                    write!(f, "{}", size.unwrap())?;
                }

                write!(f, "]")
            }
            Self::TypeParameter { name, param_ind: _ } => write!(f, "{}", name),
            Self::Reference { mutable, inner } => {
                if *mutable {
                    write!(f, "{}&mut", *inner)
                } else {
                    write!(f, "{}&", *inner)
                }
            }
            Self::Base(base) => write!(f, "{}", base),
            Self::Void => write!(f, "void"),
        }
    }
}

impl Display for BaseTypeInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ty)?;

        if !self.size_specifiers.is_empty() {
            for param in &self.size_specifiers {
                write!(f, ".{}", param)?;
            }
        }

        if !self.type_parameters.is_empty() {
            write!(f, "<{}>", fmt_list(&self.type_parameters))?;
        }

        Ok(())
    }
}
