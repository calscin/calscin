use calsc_ast::path::ElementPath;
use calsc_diagnostics::{DiagResult, DiagnosticSource};
use calsc_hir::{
    HIR_CONTEXT,
    file::{self, HIRFileContext},
    globalctx::key::GlobalContextKey,
};
use calsc_modules::path::ModulePath;

pub fn lower_ast_module_path(path: &ElementPath) -> ModulePath {
    let module_name;
    let module_path;

    if path.members.is_empty() {
        module_name = "".into();
        module_path = vec![];
    } else {
        module_name = path.members[0].clone();
        module_path = path.members[1..path.members.len()].to_vec();
    }

    ModulePath::new(module_name, module_path)
}

pub fn lower_ast_key<S: DiagnosticSource>(
    path: ElementPath,
    source: &S,
    check: bool,
    file_ctx: &mut HIRFileContext,
) -> DiagResult<GlobalContextKey> {
    let everything_but_last = path.everything_but_last();
    let last = path.last();

    if check {
        let key = lower_ast_key(everything_but_last, source, false, file_ctx)?;

        let is_type = HIR_CONTEXT.with_borrow(|ctx| {
            Ok(ctx.scope.has_entry(&key) && ctx.scope.get_entry(key.clone(), source)?.is_type())
        })?;

        let is_module = HIR_CONTEXT.with_borrow(|ctx| {
            Ok(ctx.scope.has_entry(&key) && ctx.scope.get_entry(key.clone(), source)?.is_module())
        })?;

        if is_type {
            let ty = HIR_CONTEXT.with_borrow(|ctx| {
                Ok(ctx.scope.get_entry(key.clone(), source)?.as_type(source))
            })??;

            return Ok(GlobalContextKey::new(last).associated_type(ty));
        }

        if is_module {
            let module = HIR_CONTEXT
                .with_borrow(|ctx| ctx.scope.get_entry(key.clone(), source)?.as_module(source))?;

            return Ok(GlobalContextKey::new(last).module_path(module));
        }

        return lower_ast_key(path, source, false, file_ctx);
    }

    let mut module_path;

    if path.relative {
        module_path = file_ctx.current_module.clone();
        module_path.append(lower_ast_module_path(&everything_but_last));
    } else {
        module_path = lower_ast_module_path(&everything_but_last);
    }

    Ok(GlobalContextKey::new(last).module_path(module_path))
}
