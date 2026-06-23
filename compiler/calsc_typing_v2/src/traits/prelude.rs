//! Traits related to prelude applying

use calsc_diagnostics::{DiagResult, DiagnosticSource};

use crate::types::HeldPrimitive;

/// Represents something that can apply a prelude
pub trait PreludeApplier {
    /// Registers a type inside of the prelude applier.
    fn register_type<K: DiagnosticSource>(ty: HeldPrimitive, source: &K) -> DiagResult<K>;
}
