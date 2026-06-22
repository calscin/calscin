use std::fmt::Display;

use crate::types::{MutationState, primitive::PrimitiveType};

impl Display for PrimitiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveType::Int(signed) => write!(f, "{}", if *signed { "s" } else { "u" }),
            PrimitiveType::Float => write!(f, "f"),
            PrimitiveType::Str => write!(f, "str"),
            PrimitiveType::Boolean => write!(f, "bool"),
            PrimitiveType::Size => write!(f, "size"),
        }
    }
}

impl Display for MutationState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.0 {
            return Ok(());
        }

        write!(f, " mut")
    }
}
