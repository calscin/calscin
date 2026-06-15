//! Helpers to add imported elements to the current HIR context

use calsc_diagnostics::{DiagPossible, DiagnosticSource};
use calsc_hir::{
    HIRContext,
    file::HIRFileContext,
    funcs::HIRFunction,
    globalctx::{key::GlobalContextKey, vals::GlobalContextValue},
};
use calsc_typing::{base::BaseType, tree::Type};
use calsc_utils::hash::HashedString;

pub fn import_function<S: DiagnosticSource>(
    key: GlobalContextKey,
    arguments: Vec<(HashedString, Type)>,
    return_type: Type,
    func_name: HashedString,
    file_ctx: &mut HIRFileContext,
    source: &S,
    ctx: &mut HIRContext,
) -> DiagPossible {
    let target_key =
        GlobalContextKey::new(func_name.clone()).module_path(file_ctx.current_module.clone());

    if !ctx.scope.has_entry(&key) {
        let function =
            HIRFunction::new_extern(key.clone(), None, return_type, arguments, None, false);
        let entry = GlobalContextValue::Function(function);

        ctx.scope.append(key.clone(), entry, source)?;
    }

    ctx.scope.append(
        target_key,
        GlobalContextValue::AnotherReference(key),
        source,
    )?;

    Ok(())
}

pub fn import_type<S: DiagnosticSource>(
    key: GlobalContextKey,
    base: BaseType,
    file_ctx: &mut HIRFileContext,
    source: &S,
    ctx: &mut HIRContext,
) -> DiagPossible {
    let target_key =
        GlobalContextKey::new(key.name.clone()).module_path(file_ctx.current_module.clone());

    if !ctx.scope.has_entry(&target_key) {
        let entry = GlobalContextValue::Type(base);

        ctx.scope.append(key.clone(), entry, source)?;
    }

    ctx.scope.append(
        target_key,
        GlobalContextValue::AnotherReference(key),
        source,
    )?;

    Ok(())
}
