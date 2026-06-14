//! Module dedicated to perform module importing resolving

use calsc_diagnostics::{DiagPossible, DiagnosticSource};
use calsc_hir::{
    HIR_CONTEXT,
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
) -> DiagPossible {
    let target_key =
        GlobalContextKey::new(func_name.clone()).module_path(file_ctx.current_module.clone());

    if !HIR_CONTEXT.with_borrow(|ctx| ctx.scope.has_entry(&key)) {
        let function =
            HIRFunction::new_extern(key.clone(), None, return_type, arguments, None, false);
        let entry = GlobalContextValue::Function(function);

        HIR_CONTEXT.with_borrow_mut(|ctx| ctx.scope.append(key.clone(), entry, source))?;
    }

    HIR_CONTEXT.with_borrow_mut(|ctx| {
        ctx.scope.append(
            target_key,
            GlobalContextValue::AnotherReference(key),
            source,
        )
    })?;

    Ok(())
}

pub fn import_type<S: DiagnosticSource>(
    key: GlobalContextKey,
    base: BaseType,
    file_ctx: &mut HIRFileContext,
    source: &S,
) -> DiagPossible {
    let target_key =
        GlobalContextKey::new(key.name.clone()).module_path(file_ctx.current_module.clone());

    if !HIR_CONTEXT.with_borrow(|ctx| ctx.scope.has_entry(&target_key)) {
        let entry = GlobalContextValue::Type(base);

        HIR_CONTEXT.with_borrow_mut(|ctx| ctx.scope.append(key.clone(), entry, source))?;
    }

    HIR_CONTEXT.with_borrow_mut(|ctx| {
        ctx.scope.append(
            target_key,
            GlobalContextValue::AnotherReference(key),
            source,
        )
    })?;

    Ok(())
}
