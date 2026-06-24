use calsc_diagnostics::{DiagPossible, DiagnosticSource};
use calsc_hir::{
    BUILD_CACHE, HIRContext,
    funcs::HIRFunction,
    globalctx::{key::GlobalContextKey, vals::GlobalContextValue},
};
use calsc_modules::{
    lazy::LazyLoadedType, path::ModulePath, tree::entry::TreeModule, visibility::Visibility,
};
use calsc_utils::hash::HashedString;

pub fn import_function<S: DiagnosticSource>(
    return_type: LazyLoadedType,
    arguments: Vec<(HashedString, LazyLoadedType)>,
    path_to_append: ModulePath,
    ctx: &mut HIRContext,
    origin: &S,
) -> DiagPossible {
    let name = path_to_append.last();
    let key = GlobalContextKey::new(name.clone()).module_path(path_to_append.everything_but_last());

    ctx.scope.append(
        key.clone(),
        GlobalContextValue::Function(HIRFunction::new_extern(
            key,
            return_type,
            arguments,
            triple_dot_position,
            is_main_function,
        )),
        Visibility::Uncopiable,
        origin,
    )?;
}

pub fn import_module(
    module: &TreeModule,
    path_to_append: ModulePath,
    ctx: &mut HIRContext,
) -> DiagPossible {
}
