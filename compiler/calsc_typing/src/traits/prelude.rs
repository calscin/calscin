//! Traits related to prelude applying

use calsc_diagnostics::{DiagPossible, DiagnosticSource};
use calsc_utils::hash::HashedString;

use crate::types::primitive::PrimitiveType;

/// Represents something that can apply a prelude
pub trait PreludeApplier {
    /// Registers a type inside of the prelude applier.
    fn register_type<K: DiagnosticSource>(
        &mut self,
        name: HashedString,
        ty: PrimitiveType,
        source: &K,
    ) -> DiagPossible;
}
