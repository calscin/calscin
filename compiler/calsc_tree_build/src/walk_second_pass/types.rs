use calsc_ast::types::ASTType;
use calsc_diagnostics::{DiagPossible, DiagnosticSource};
use calsc_modules::path::ModulePath;
use calsc_utils::hash::HashedString;

use crate::{ctx::TreeBuildingCtx, utils::resolve_path};

pub(crate) fn get_typing_deps_inner<S: DiagnosticSource>(
    ty: &ASTType,
    ctx: &TreeBuildingCtx,
    vec: &mut Vec<ModulePath>,
    type_params: &Vec<HashedString>,
    source: &S,
) -> DiagPossible {
    match ty {
        ASTType::Array(size, inner) => {
            if size.is_none() {
                return Ok(());
            }

            get_typing_deps_inner(&*inner, ctx, vec, type_params, source)
        }

        ASTType::Generic(name, _, _) => {
            if type_params.contains(&name.members[0]) {
                return Ok(());
            }

            vec.push(resolve_path(name.clone(), ctx, source)?);

            Ok(())
        }

        _ => Ok(()),
    }
}

pub(crate) fn get_semantic_deps_inner<S: DiagnosticSource>(
    ty: &ASTType,
    ctx: &TreeBuildingCtx,
    type_params: &Vec<HashedString>,
    vec: &mut Vec<ModulePath>,
    source: &S,
) -> DiagPossible {
    match ty {
        ASTType::Array(_, inner) => get_semantic_deps_inner(&*inner, ctx, type_params, vec, source),
        ASTType::Pointer(_, inner) => {
            get_semantic_deps_inner(&*inner, ctx, type_params, vec, source)
        }
        ASTType::Reference(_, inner) => {
            get_semantic_deps_inner(&*inner, ctx, type_params, vec, source)
        }
        ASTType::Generic(name, _, _) => {
            if type_params.contains(&name.members[0]) {
                return Ok(());
            }

            vec.push(resolve_path(name.clone(), ctx, source)?);

            Ok(())
        }
        ASTType::Void => Ok(()),
    }
}
