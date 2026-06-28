use calsc_ast::path::ElementPath;
use calsc_diagnostics::{DiagPossible, DiagnosticSource};
use calsc_hir::{
    BUILD_CACHE, HIRContext,
    file::HIRFileContext,
    funcs::HIRFunction,
    globalctx::{key::GlobalContextKey, vals::GlobalContextValue},
};
use calsc_modules::{
    lazy::{LazyLoadedType, func::LazyLoadedFunction},
    path::ModulePath,
    tree::entry::{ModuleTreeEntry, TreeModule},
    visibility::Visibility,
};
use calsc_utils::hash::HashedString;

use crate::stage2::types::lower_module_path_type;

pub mod lower;

pub fn import_function<S: DiagnosticSource>(
    func: LazyLoadedFunction,
    path_to_append: ModulePath,
    ctx: &mut HIRContext,
    origin: &S,
) -> DiagPossible {
    let group = ctx.type_ctx.type_params.start_param_group();

    let name = path_to_append.last();

    let mut owned_type_params = vec![];

    for type_parameter in func.type_paramers {
        let id = ctx
            .type_ctx
            .type_params
            .append_type_param(type_parameter, origin)?;

        owned_type_params.push(id);
    }

    let key = GlobalContextKey::new(name.clone()).module_path(path_to_append.everything_but_last());

    let return_type = lower_module_path_type(func.return_type, origin, ctx)?;
    let arguments: Vec<_> = func
        .arguments
        .iter()
        .map(|(name, ty)| {
            lower_module_path_type(ty.clone(), origin, ctx).map(|ty| (name.clone(), ty))
        })
        .collect::<Result<_, _>>()?;

    let mut func = HIRFunction::new_imported(key.clone(), return_type, arguments, false);
    func.type_parameters = owned_type_params;

    ctx.scope.append(
        key.clone(),
        GlobalContextValue::Function(func),
        Visibility::Uncopiable,
        origin,
    )?;

    ctx.type_ctx.type_params.end_group(group);

    Ok(())
}

pub fn import_type<S: DiagnosticSource>(
    path_to_append: ModulePath,
    ctx: &mut HIRContext,
    origin: &S,
) -> DiagPossible {
    let name = path_to_append.last();
    let key = GlobalContextKey::new(name.clone()).module_path(path_to_append.everything_but_last());

    let ty = BUILD_CACHE.with_borrow(|cache| cache.type_storage.map[&path_to_append].clone());

    ctx.scope.append(
        key,
        GlobalContextValue::Type(ty),
        Visibility::Uncopiable,
        origin,
    )?;

    Ok(())
}

pub fn import_module<S: DiagnosticSource>(
    module: TreeModule,
    path_to_append: ModulePath,
    ctx: &mut HIRContext,
    origin: &S,
) -> DiagPossible {
    for element in module.children {
        let mut path_to_append_to = path_to_append.clone();
        path_to_append_to.append_single_bit(element.0);

        import_element(element.1, path_to_append_to, ctx, origin)?;
    }

    Ok(())
}

pub fn import_element<S: DiagnosticSource>(
    element: ModuleTreeEntry,
    path_to_append: ModulePath,
    ctx: &mut HIRContext,
    origin: &S,
) -> DiagPossible {
    match element {
        ModuleTreeEntry::FilledFunction(func) => import_function(func, path_to_append, ctx, origin),

        ModuleTreeEntry::FilledType(_) => import_type(path_to_append, ctx, origin),

        ModuleTreeEntry::Module(module) => import_module(module, path_to_append, ctx, origin),

        _ => panic!("Entry isn't filled yet"),
    }
}

pub fn lower_hir_key(path: ElementPath, hir_ctx: &HIRFileContext) -> ModulePath {
    // We don't need to care about the prelude here since it'll be automatically imported anyways

    if path.relative {
        let mut hir_path = hir_ctx.current_module.clone();

        for elem in path.members {
            hir_path.append_single_bit(elem);
        }

        hir_path
    } else {
        ModulePath::new(
            path.members[0].clone(),
            path.members[1..path.members.len()].to_vec(),
        )
    }
}
