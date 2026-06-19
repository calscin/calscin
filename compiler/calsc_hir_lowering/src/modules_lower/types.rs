use calsc_ast::{
    nodes::{ASTNode, ASTNodeKind},
    types::ASTType,
};
use calsc_diagnostics::{
    DiagPossible, DiagResult, DiagnosticSource,
    diags::errors::{build_expected_entry_type, build_internal_hir_node_leaked},
};
use calsc_hir::{BUILD_CACHE, file::HIRFileContext};
use calsc_modules::{
    lazy::LazyLoadedTypeLike,
    path::ModulePath,
    tree::{ModuleTree, entry::ModuleTreeEntry},
};

use calsc_typing::{
    MutableFieldHavingType,
    base::{
        BaseType, instance::BaseTypeInstance, kind::BaseTypeKind, structs::BaseStructContainer,
    },
    tree::Type,
};
use calsc_utils::hash::HashedCounter;

use crate::stage0::types::lower_stage0_key;

pub fn lower_type_from_tree(path: ModulePath, tree: &ModuleTree) -> DiagPossible {
    let already_built = BUILD_CACHE.with_borrow(|cache| cache.type_storage.map.contains_key(&path));

    if already_built {
        return Ok(()); // No need to do anything else if its already built.
    }

    let source = BUILD_CACHE.with_borrow(|cache| cache.nodes_to_entries[&path][0].clone());

    let r = tree.traverse_to(path.clone(), &source)?;

    if let ModuleTreeEntry::Type(ty) = r {
        let mut dependencies = HashedCounter::new();

        ty.get_dependencies(tree, &mut dependencies, &source)?;

        // Lower each dependency first so that the type can be safely resolved
        for dependency in &dependencies.map {
            lower_type_from_tree(dependency.0.clone(), tree)?;
        }

        // We first build a HIR file context with the current module path to allow for same-module resolution
        let module_path = path.everything_but_last();

        let hir_file_ctx = HIRFileContext {
            current_module: module_path,
            lazy_imports: vec![],
        };

        // We then lower each part of the type by lowering each related AST node.
        let related_nodes = BUILD_CACHE.with_borrow(|cache| cache.nodes_to_entries[&path].clone());

        for node in related_nodes {
            lower_type_node(path.clone(), tree, node, &hir_file_ctx, &source)?;
        }
    } else {
        return Err(build_expected_entry_type(&"type".to_string(), &path, &source).into());
    }

    Ok(())
}

pub fn lower_type<S: DiagnosticSource>(
    ty: ASTType,
    tree: &ModuleTree,
    hir_file_ctx: &HIRFileContext,
    source: &S,
) -> DiagResult<Type> {
    match ty {
        ASTType::Array(size, inner) => Ok(Type::Array {
            size,
            inner: Box::new(lower_type(*inner, tree, hir_file_ctx, source)?),
        }),

        ASTType::Reference(mutable, inner) => Ok(Type::Reference {
            mutable,
            inner: Box::new(lower_type(*inner, tree, hir_file_ctx, source)?),
        }),

        ASTType::Generic(name, size_specs, type_parameters) => {
            let mut lowered_type_params = vec![];
            let mut size_specifiers = vec![];

            for param in type_parameters {
                lowered_type_params.push(lower_type(param, tree, hir_file_ctx, source)?);
            }

            if size_specs.is_some() {
                size_specifiers.push(size_specs.unwrap());
            }

            let (mut path, element_name) = lower_stage0_key(name, hir_file_ctx, tree);
            path.append_single_bit(element_name);

            let raw_type = BUILD_CACHE.with_borrow(|state| state.type_storage.map[&path].clone());

            let instance = BaseTypeInstance::new(raw_type, size_specifiers, lowered_type_params);

            Ok(Type::Base(instance))
        }

        ASTType::Void => Ok(Type::Void),
    }
}

pub fn lower_type_node<S: DiagnosticSource>(
    path: ModulePath,
    tree: &ModuleTree,
    node: ASTNode,
    hir_file_ctx: &HIRFileContext,
    source: &S,
) -> DiagPossible {
    match node.kind {
        ASTNodeKind::StructDeclaration { .. } => {
            lower_type_struct_decl(path, tree, node, hir_file_ctx, source)
        }

        _ => return Err(build_internal_hir_node_leaked(&node, source).into()),
    }
}

pub fn lower_type_struct_decl<S: DiagnosticSource>(
    path: ModulePath,
    tree: &ModuleTree,
    node: ASTNode,
    hir_file_ctx: &HIRFileContext,
    source: &S,
) -> DiagPossible {
    if let ASTNodeKind::StructDeclaration {
        name,
        type_params,
        fields,
        visibility: _,
    } = node.kind.clone()
    {
        let mut container = BaseStructContainer::new(name);

        for field in fields {
            container.add_field(
                field.1,
                lower_type(field.0, tree, hir_file_ctx, source)?,
                source,
            )?;
        }

        let mut base_type = BaseType::new(BaseTypeKind::Struct(container));

        for type_param in type_params {
            base_type.append_type_parameter(type_param, source)?;
        }

        BUILD_CACHE.with_borrow_mut(|cache| cache.type_storage.map.insert(path, base_type));

        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(&node, source).into());
    }
}
