use std::path::PathBuf;

use calsc_diagnostics::{DiagPossible, DiagnosticSource};
use calsc_modules::{
    path::ModulePath,
    treev2::{entry::TreeEntryKind, module::TreeModule},
};

use crate::ctx::TreeBuildingCtx;

pub fn apply_prelude<S: DiagnosticSource>(ctx: &mut TreeBuildingCtx, source: &S) -> DiagPossible {
    ctx.tree.append_entry(
        &ModulePath::new_prelude_path(vec![]),
        TreeEntryKind::Module(TreeModule::new("prelude".into(), PathBuf::from(""))),
        &mut ctx.arena,
        source,
    )?;

    ctx.tree.append_entry(
        &ModulePath::new_prelude_path(vec!["s".into()]),
        TreeEntryKind::PrimitiveType,
        &mut ctx.arena,
        source,
    )?;

    ctx.tree.append_entry(
        &ModulePath::new_prelude_path(vec!["u".into()]),
        TreeEntryKind::PrimitiveType,
        &mut ctx.arena,
        source,
    )?;

    ctx.tree.append_entry(
        &ModulePath::new_prelude_path(vec!["f".into()]),
        TreeEntryKind::PrimitiveType,
        &mut ctx.arena,
        source,
    )?;

    ctx.tree.append_entry(
        &ModulePath::new_prelude_path(vec!["size".into()]),
        TreeEntryKind::PrimitiveType,
        &mut ctx.arena,
        source,
    )?;

    ctx.tree.append_entry(
        &ModulePath::new_prelude_path(vec!["str".into()]),
        TreeEntryKind::PrimitiveType,
        &mut ctx.arena,
        source,
    )?;

    ctx.tree.append_entry(
        &ModulePath::new_prelude_path(vec!["bool".into()]),
        TreeEntryKind::PrimitiveType,
        &mut ctx.arena,
        source,
    )
}
