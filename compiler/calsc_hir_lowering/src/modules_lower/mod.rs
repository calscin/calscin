use calsc_diagnostics::{DiagPossible, panics::PanicDiagnosticSource};
use calsc_hir::BUILD_CACHE;
use calsc_modules::{
    path::ModulePath,
    tree::{ModuleTree, collect::ModuleTreeCollector},
};
use calsc_typing::{ctx::TypeCtx, prelude::apply_prelude};

use crate::modules_lower::types::lower_type_from_tree;

pub mod types;

pub fn lower_types_from_stage_0(tree: &ModuleTree) -> DiagPossible {
    // Setup the prelude environment.

    let mut type_ctx = TypeCtx::new();

    BUILD_CACHE
        .with_borrow_mut(|cache| apply_prelude(&mut cache.type_storage, &PanicDiagnosticSource()));

    let mut types = vec![];

    tree.collect_entries(
        &|f| f.is_type(),
        ModulePath::new("".into(), vec![]),
        &mut types,
    );

    for ty in types {
        lower_type_from_tree(ty.1, tree, &mut type_ctx)?;
    }

    Ok(())
}
