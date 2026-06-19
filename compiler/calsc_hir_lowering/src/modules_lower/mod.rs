use calsc_diagnostics::{DiagPossible, DiagnosticSource};
use calsc_modules::{
    path::ModulePath,
    tree::{ModuleTree, collect::ModuleTreeCollector},
};

use crate::modules_lower::{
    prelude::apply_prelude_to_module_tree_lowering, types::lower_type_from_tree,
};

pub mod prelude;
pub mod types;

pub fn lower_types_from_stage_0<S: DiagnosticSource>(
    tree: &ModuleTree,
    source: &S,
) -> DiagPossible {
    // Setup the prelude environment.

    apply_prelude_to_module_tree_lowering();

    let mut types = vec![];

    tree.collect_entries(
        &|f| f.is_type(),
        ModulePath::new("".into(), vec![]),
        &mut types,
    );

    for ty in types {
        lower_type_from_tree(ty.1, tree, source)?;
    }

    Ok(())
}
