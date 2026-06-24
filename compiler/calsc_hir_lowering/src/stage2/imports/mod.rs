use calsc_ast::path::ElementPath;
use calsc_diagnostics::{DiagPossible, DiagnosticSource};
use calsc_hir::{
    BUILD_CACHE, HIRContext,
    file::HIRFileContext,
    funcs::HIRFunction,
    globalctx::{key::GlobalContextKey, vals::GlobalContextValue},
};
use calsc_modules::{
    lazy::LazyLoadedType,
    path::ModulePath,
    tree::entry::{ModuleTreeEntry, TreeModule},
    visibility::Visibility,
};
use calsc_utils::hash::HashedString;

use crate::stage2::types::lower_module_path_type;

pub mod lower;

pub fn import_function<S: DiagnosticSource>(
    return_type: LazyLoadedType,
    arguments: Vec<(HashedString, LazyLoadedType)>,
    path_to_append: ModulePath,
    ctx: &mut HIRContext,
    origin: &S,
) -> DiagPossible {
    let name = path_to_append.last();

    println!("{}", path_to_append);

    let key = GlobalContextKey::new(name.clone()).module_path(path_to_append.everything_but_last());

    let return_type = lower_module_path_type(return_type, origin, ctx)?;
    let arguments: Vec<_> = arguments
        .iter()
        .map(|(name, ty)| {
            lower_module_path_type(ty.clone(), origin, ctx).map(|ty| (name.clone(), ty))
        })
        .collect::<Result<_, _>>()?;

    ctx.scope.append(
        key.clone(),
        GlobalContextValue::Function(HIRFunction::new_extern(
            key,
            return_type,
            arguments,
            None,
            false,
        )),
        Visibility::Uncopiable,
        origin,
    )?;

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

        println!("Element path: {}", path_to_append_to);

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
        ModuleTreeEntry::FilledFunction(return_type, arguments) => {
            import_function(return_type, arguments, path_to_append, ctx, origin)
        }

        ModuleTreeEntry::FilledType(_) => import_type(path_to_append, ctx, origin),

        ModuleTreeEntry::Module(module) => import_module(module, path_to_append, ctx, origin),

        _ => panic!("Entry isn't filled yet"),
    }
}

pub fn lower_hir_key(path: ElementPath, hir_ctx: &HIRFileContext) -> ModulePath {
    // We don't need to care about the prelude here since it'll be automatically imported anyways

    println!("{:#?}", path);

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
