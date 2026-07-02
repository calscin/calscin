use std::path::PathBuf;

use calsc_ast::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind},
};
use calsc_diagnostics::{
    DiagPossible, DiagResult, DiagnosticSource,
    diags::errors::{build_module_no_files, build_multiple_files_one_module},
};
use calsc_utils::hash::HashedString;

use crate::ctx::TreeBuildingCtx;

pub fn discover_files(
    ast: &ASTContext,
    ctx: &mut TreeBuildingCtx,
    parent_buff: PathBuf,
) -> DiagResult<Vec<PathBuf>> {
    let mut paths = vec![];

    for node in &ast.tree {
        let node = ast.nodes.get(node);

        if let ASTNodeKind::Module {
            name,
            is_bodied,
            body: _,
        } = &node.kind
        {
            if *is_bodied {
                continue;
            }

            let path = get_file_path_for_module_name(&parent_buff, name, node)?;

            println!("Detected {:#?}", path);

            paths.push(path);
        }
    }

    Ok(paths)
}

/// Gets the file path for the given module name
pub fn get_file_path_for_module_name<S: DiagnosticSource>(
    parent_path: &PathBuf,
    name: &HashedString,
    source: &S,
) -> DiagResult<PathBuf> {
    let path_1 = PathBuf::from(format!("{}.cal", name));
    let path_2 = PathBuf::from(format!("{}/module.cal", name));

    let path_1 = parent_path.join(path_1);
    let path_2 = parent_path.join(path_2);

    if path_1.exists() == path_2.exists() {
        if !path_1.exists() {
            return Err(build_module_no_files(source, name).into());
        } else {
            return Err(build_multiple_files_one_module(source, name).into());
        }
    }

    if path_1.exists() {
        Ok(path_1)
    } else {
        Ok(path_2)
    }
}
