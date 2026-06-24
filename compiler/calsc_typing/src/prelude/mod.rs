//! Represents the type prelude. Allows to use types

use calsc_diagnostics::{DiagPossible, DiagnosticSource};

use crate::{traits::PreludeApplier, types::primitive::PrimitiveType};

pub fn apply_prelude<K: PreludeApplier, S: DiagnosticSource>(
    applier: &mut K,
    source: &S,
) -> DiagPossible {
    applier.register_type("s".into(), PrimitiveType::Int(true), source)?;
    applier.register_type("u".into(), PrimitiveType::Int(false), source)?;
    applier.register_type("f".into(), PrimitiveType::Float, source)?;
    applier.register_type("size".into(), PrimitiveType::Size, source)?;
    applier.register_type("str".into(), PrimitiveType::Str, source)?;
    applier.register_type("bool".into(), PrimitiveType::Boolean, source)
}
