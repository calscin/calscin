//! Traits related to types

use calsc_diagnostics::{
    DiagPossible, DiagResult, DiagnosticSource,
    diags::errors::{build_expected_field_type, build_missing_field},
};
use calsc_utils::{display_with_to_string, hash::HashedString};

use crate::{TypingInterner, ctx::TypeCtx, types::TypeKind};

/// A type that contains fields.
pub trait FieldedType {
    /// Determines if the type has the field with the given name.
    fn has_field(&self, name: &HashedString, ctx: &TypeCtx, interner: &TypingInterner) -> bool;

    /// Gets the field type corresponding to the file with the given name.
    ///
    /// # Panics
    /// This function will panic if the field doesn't exist, this is why it is unsafe. Consider using [`FieldedType::get_field_safe`] instead.
    ///
    unsafe fn get_field(
        &self,
        field: &HashedString,
        ctx: &TypeCtx,
        interner: &TypingInterner,
    ) -> TypeKind;

    /// gets the list of field names
    fn get_fields(&self, ctx: &TypeCtx, interner: &TypingInterner) -> Vec<HashedString>;

    fn get_field_index(
        &self,
        field: &HashedString,
        ctx: &TypeCtx,
        interner: &TypingInterner,
    ) -> usize;

    /// Safely gets the type of a field.
    ///
    /// # Errors
    /// This function will error if the type is not found.
    ///
    fn get_field_safe<S: DiagnosticSource>(
        &self,
        field: &HashedString,
        ctx: &TypeCtx,
        interner: &TypingInterner,
        source: &S,
    ) -> DiagResult<TypeKind> {
        if !self.has_field(field, ctx, interner) {
            return Err(build_missing_field(field, source).into());
        }

        unsafe { Ok(self.get_field(field, ctx, interner)) }
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
        interner: &TypingInterner,
        source: &S,
    ) -> DiagPossible {
        let self_ty = self.get_field_safe(field, ctx, interner, source)?;

        if self_ty != *ty {
            return Err(build_expected_field_type(
                field,
                &display_with_to_string(ty, &interner),
                &display_with_to_string(&self_ty, &interner),
                source,
            )
            .into());
        }

        Ok(())
    }
}
