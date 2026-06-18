use calsc_diagnostics::{DiagPossible, DiagnosticSource};
use calsc_modules::{
    lazy::raw::{LazyLoadedRawType, LazyLoadedRawTypeKind},
    path::ModulePath,
    tree::{ModuleTree, entry::ModuleTreeEntry},
};

pub fn apply_stage0_prelude<S: DiagnosticSource>(
    tree: &mut ModuleTree,
    source: &S,
) -> DiagPossible {
    tree.traverse_to_append(
        ModulePath::new_prelude_path(vec!["bool".into()]),
        ModuleTreeEntry::Type(LazyLoadedRawType::new(LazyLoadedRawTypeKind::Simple)),
        source,
    )?;

    tree.traverse_to_append(
        ModulePath::new_prelude_path(vec!["s".into()]),
        ModuleTreeEntry::Type(LazyLoadedRawType::new(LazyLoadedRawTypeKind::Simple)),
        source,
    )?;

    tree.traverse_to_append(
        ModulePath::new_prelude_path(vec!["u".into()]),
        ModuleTreeEntry::Type(LazyLoadedRawType::new(LazyLoadedRawTypeKind::Simple)),
        source,
    )?;

    tree.traverse_to_append(
        ModulePath::new_prelude_path(vec!["f".into()]),
        ModuleTreeEntry::Type(LazyLoadedRawType::new(LazyLoadedRawTypeKind::Simple)),
        source,
    )?;

    tree.traverse_to_append(
        ModulePath::new_prelude_path(vec!["uf".into()]),
        ModuleTreeEntry::Type(LazyLoadedRawType::new(LazyLoadedRawTypeKind::Simple)),
        source,
    )?;

    tree.traverse_to_append(
        ModulePath::new_prelude_path(vec!["str".into()]),
        ModuleTreeEntry::Type(LazyLoadedRawType::new(LazyLoadedRawTypeKind::Simple)),
        source,
    )?;

    tree.traverse_to_append(
        ModulePath::new_prelude_path(vec!["char".into()]),
        ModuleTreeEntry::Type(LazyLoadedRawType::new(LazyLoadedRawTypeKind::Simple)),
        source,
    )
}
