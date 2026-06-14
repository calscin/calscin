use calsc_ast::path::ElementPath;
use calsc_diagnostics::{DiagResult, DiagnosticSource};
use calsc_hir::{HIR_CONTEXT, globalctx::key::GlobalContextKey};
use calsc_modules::path::ModulePath;

pub fn lower_ast_module_path(path: &ElementPath) -> ModulePath {
    let module_name = path.members[0].clone();
    let module_path = path.members[1..path.members.len()].to_vec();

    ModulePath::new(module_name, module_path)
}

pub fn lower_ast_key<S: DiagnosticSource>(
    path: ElementPath,
    source: &S,
    check_for_type: bool,
) -> DiagResult<GlobalContextKey> {
    let everything_but_last = path.everything_but_last();
    let last = path.last();

    if check_for_type {
        let key = lower_ast_key(everything_but_last, source, false)?;

        let is_type = HIR_CONTEXT.with_borrow(|ctx| {
            Ok(ctx.scope.has_entry(&key) && ctx.scope.get_entry(key.clone(), source)?.is_type())
        })?;

        if is_type {
            let ty = HIR_CONTEXT.with_borrow(|ctx| {
                Ok(ctx.scope.get_entry(key.clone(), source)?.as_type(source))
            })??;

            return Ok(GlobalContextKey::new(last).associated_type(ty));
        }

        return lower_ast_key(path, source, false);
    }

    let module_path = lower_ast_module_path(&everything_but_last);

    let key = GlobalContextKey::new(last).module_path(module_path);
    Ok(key)
}
