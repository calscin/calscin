//! Traits related to types

use calsc_diagnostics::{
    DiagPossible, DiagResult, DiagnosticSource,
    diags::errors::{build_expected_field_type, build_missing_field},
};
use calsc_utils::{display_with_to_string, hash::HashedString};

use crate::{ctx::TypeCtx, types::TypeKind};

/// A type that contains fields.
pub trait FieldedType {
    /// Determines if the type has the field with the given name.
    fn has_field(&self, name: &HashedString, ctx: &TypeCtx) -> bool;

    /// Gets the field type corresponding to the file with the given name.
    ///
    /// # Panics
    /// This function will panic if the field doesn't exist, this is why it is unsafe. Consider using [`FieldedType::get_field_safe`] instead.
    ///
    unsafe fn get_field(&self, field: &HashedString, ctx: &TypeCtx) -> TypeKind;

    /// Safely gets the type of a field.
    ///
    /// # Errors
    /// This function will error if the type is not found.
    ///
    fn get_field_safe<S: DiagnosticSource>(
        &self,
        field: &HashedString,
        ctx: &TypeCtx,
        source: &S,
    ) -> DiagResult<TypeKind> {
        if !self.has_field(field, ctx) {
            return Err(build_missing_field(field, source).into());
        }

        unsafe { Ok(self.get_field(field, ctx)) }
    }

    /// Enforces a field to exist with the given type.
    ///
    /// # Errors
    /// This function will error if the field doesn't eixst.
    /// This function will error if the field isn't of the given type
    ///
    fn enforce_field<S: DiagnosticSource>(
        &self,
        field: &HashedString,
        ty: &TypeKind,
        ctx: &TypeCtx,
        source: &S,
    ) -> DiagPossible {
        let self_ty = self.get_field_safe(field, ctx, source)?;

        if self_ty != *ty {
            return Err(build_expected_field_type(
                field,
                &display_with_to_string(ty, &ctx.type_kind_arena),
                &display_with_to_string(&self_ty, &ctx.type_kind_arena),
                source,
            )
            .into());
        }

        Ok(())
    }
}
