use std::fmt::Display;

use crate::base::{BaseType, kind::BaseTypeKind};

impl Display for BaseTypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Boolean => "bool",
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
        }

        Ok(())
    }
}
