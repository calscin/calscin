use calsc_ast::path::ElementPath;
use calsc_diagnostics::{DiagResult, DiagnosticSource};
use calsc_hir::{HIRContext, file::HIRFileContext, globalctx::key::GlobalContextKey};
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
    ctx: &mut HIRContext,
) -> DiagResult<GlobalContextKey> {
    let everything_but_last = path.everything_but_last();
    let last = path.last();

    if check && everything_but_last.members.len() >= 1 {
        let key = lower_ast_key(everything_but_last, source, false, file_ctx, ctx)?;

        let is_entry = ctx.scope.has_entry(&key);

        let is_module = is_entry
            && ctx
                .scope
                .get_entry(key.clone(), &file_ctx.current_module, source)?
                .is_module();

        let is_type = is_entry
            && ctx
                .scope
                .get_entry(key.clone(), &file_ctx.current_module, source)?
                .is_type();

        if is_module {
            let module = ctx
                .scope
                .get_entry(key.clone(), &file_ctx.current_module, source)?
                .as_module(source)?;

            return Ok(GlobalContextKey::new(last).module_path(module));
        }

        if is_type {
            return Ok(GlobalContextKey::new(last).associated_type(key));
        }

        return lower_ast_key(path, source, false, file_ctx, ctx);
    }

    let mut module_path;

    if path.relative {
        module_path = file_ctx.current_module.clone();

        // Check if prelude is found: If found: prelude, takes priority:
        {
            let key = GlobalContextKey::new(last.clone())
                .module_path(ModulePath::new_prelude_path(vec![]));

            if ctx.scope.has_entry(&key) {
                return Ok(key);
            }
        }

        module_path.append(lower_ast_module_path(&everything_but_last));
    } else {
        module_path = lower_ast_module_path(&everything_but_last);
    }

    Ok(GlobalContextKey::new(last).module_path(module_path))
}
