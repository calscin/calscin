use calsc_diagnostics::{DiagPossible, DiagnosticSource, diags::errors::build_expected_entry_type};
use calsc_modules::{
    lazy::raw::{LazyLoadedRawType, LazyLoadedRawTypeKind},
    path::ModulePath,
    tree::{ModuleTree, entry::ModuleTreeEntry},
};

pub fn apply_stage0_prelude<S: DiagnosticSource>(
    tree: &mut ModuleTree,
    source: &S,
) -> DiagPossible {
    let mod_ref =
        tree.traverse_mutably_to(ModulePath::new_module_tree_prelude_path(vec![]), source)?;

    if let ModuleTreeEntry::Module(module) = mod_ref {
        module.imported = true;
    } else {
        return Err(build_expected_entry_type(
            &"module".to_string(),
            &ModulePath::new_module_tree_prelude_path(vec![]),
            source,
        )
        .into());
    }

    tree.traverse_to_append(
        ModulePath::new_module_tree_prelude_path(vec!["bool".into()]),
        ModuleTreeEntry::FilledType(LazyLoadedRawType::new(LazyLoadedRawTypeKind::Simple)),
        source,
    )?;

    tree.traverse_to_append(
        ModulePath::new_module_tree_prelude_path(vec!["s".into()]),
        ModuleTreeEntry::FilledType(LazyLoadedRawType::new(LazyLoadedRawTypeKind::Simple)),
        source,
    )?;

    tree.traverse_to_append(
        ModulePath::new_module_tree_prelude_path(vec!["u".into()]),
        ModuleTreeEntry::FilledType(LazyLoadedRawType::new(LazyLoadedRawTypeKind::Simple)),
        source,
    )?;

    tree.traverse_to_append(
        ModulePath::new_module_tree_prelude_path(vec!["f".into()]),
        ModuleTreeEntry::FilledType(LazyLoadedRawType::new(LazyLoadedRawTypeKind::Simple)),
        source,
    )?;

    tree.traverse_to_append(
        ModulePath::new_module_tree_prelude_path(vec!["uf".into()]),
        ModuleTreeEntry::FilledType(LazyLoadedRawType::new(LazyLoadedRawTypeKind::Simple)),
        source,
    )?;

    tree.traverse_to_append(
        ModulePath::new_module_tree_prelude_path(vec!["str".into()]),
        ModuleTreeEntry::FilledType(LazyLoadedRawType::new(LazyLoadedRawTypeKind::Simple)),
        source,
    )?;

    tree.traverse_to_append(
        ModulePath::new_module_tree_prelude_path(vec!["char".into()]),
        ModuleTreeEntry::FilledType(LazyLoadedRawType::new(LazyLoadedRawTypeKind::Simple)),
        source,
    )?;

    tree.traverse_to_append(
        ModulePath::new_module_tree_prelude_path(vec!["size".into()]),
        ModuleTreeEntry::FilledType(LazyLoadedRawType::new(LazyLoadedRawTypeKind::Simple)),
        source,
    )
}
