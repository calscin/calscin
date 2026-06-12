use std::path::PathBuf;

use calsc_diagnostics::{DiagPossible, PosDiagnosticSource};
use calsc_hir::HIRContext;
use calsc_state::build::BuildTargetMode;
use calsc_utils::pos::FilePosition;
use remir::module::Module;
use remir_llvm::{LLVMBridge, build_llvm, compile_llvm};

use crate::funcs::{lower_hir_function_decl, lower_hir_function_decl_none};

pub mod body;
pub mod control;
pub mod funcs;
pub mod indexes;
pub mod range;
pub mod reads;
pub mod result;
pub mod types;
pub mod values;
pub mod vars;
pub mod writes;

pub fn lower_hir_context(ctx: HIRContext, module: &mut Module) -> DiagPossible {
    let dummy_pos = PosDiagnosticSource::new(FilePosition::default(), FilePosition::default()); // This is okay since we are sure that as_function won't fail

    // First round: registering functions
    for key in ctx.scope.key_to_ind.keys() {
        let entry = ctx.scope.get_entry(key.clone(), &dummy_pos)?;

        if entry.is_function() {
            let func = entry.as_function(&dummy_pos)?;

            lower_hir_function_decl_none(
                key.clone(),
                func.arguments.iter().map(|f| f.1.clone()).collect(),
                func.return_type.clone(),
                func.is_main_function,
                module,
            )?;
        }
    }

    // Second round: lowering bodies
    for key in ctx.scope.key_to_ind.keys() {
        let entry = ctx.scope.get_entry(key.clone(), &dummy_pos)?;

        if entry.is_function() {
            let func = entry.as_function(&dummy_pos)?;

            if func.impl_node.is_some() {
                lower_hir_function_decl(func.impl_node.clone().unwrap(), &ctx, module)?;
            }
        }
    }

    Ok(())
}

pub fn compile_file(
    ctx: HIRContext,
    out: PathBuf,
    module_name: String,
    target: BuildTargetMode,
) -> DiagPossible {
    let mut module = Module::new(module_name.clone());

    lower_hir_context(ctx, &mut module)?;

    if !target.requires_vendor_ir() {
        module.save_to_file(out).unwrap();

        return Ok(());
    }

    let mut bridge = LLVMBridge::new();
    build_llvm(&mut bridge, &mut module)?;

    bridge.modules[&module_name].print_to_file(out).unwrap();

    Ok(())
}

#[deprecated]
pub fn print_context_as_remir(
    ctx: HIRContext,
    out: PathBuf,
    module_name: String,
    remir: bool,
) -> DiagPossible {
    let mut module = Module::new(module_name.clone());

    lower_hir_context(ctx, &mut module)?;

    //lazy_pass(&mut module).unwrap();

    if remir {
        module.save_to_file(out).unwrap();

        return Ok(());
    }

    let mut bridge = LLVMBridge::new();
    build_llvm(&mut bridge, &mut module)?;

    bridge.modules[&module_name].print_to_file(out).unwrap();

    Ok(())
}

#[deprecated]
pub fn build_context_to_object_file(
    ctx: HIRContext,
    module_name: String,
    path: PathBuf,
    pie: bool,
) -> DiagPossible {
    let mut module = Module::new(module_name);

    lower_hir_context(ctx, &mut module)?;

    let mut bridge = LLVMBridge::new();

    compile_llvm(
        &mut bridge,
        &mut module,
        remir::OptimizationLevel::Default,
        path,
        pie,
    )
}
