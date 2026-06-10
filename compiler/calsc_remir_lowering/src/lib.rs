use std::path::PathBuf;

use calsc_diagnostics::{DiagPossible, PosDiagnosticSource};
use calsc_hir::HIRContext;
use calsc_utils::pos::FilePosition;
use remir::module::Module;
use remir_llvm::{LLVMBridge, build_llvm, compile_llvm};

use crate::funcs::{lower_hir_function_decl, lower_hir_function_decl_none};

pub mod assigns;
pub mod body;
pub mod control;
pub mod funcs;
pub mod range;
pub mod result;
pub mod types;
pub mod values;
pub mod vars;

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
                func.local_context.as_ref().unwrap().is_main_function,
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

pub fn print_context_as_mir(ctx: HIRContext) -> DiagPossible {
    let mut module = Module::new("sample_mod".to_string());

    lower_hir_context(ctx, &mut module)?;

    //lazy_pass(&mut module).unwrap();

    module.save_to_file(PathBuf::from("test.remir")).unwrap();

    let mut bridge = LLVMBridge::new();
    build_llvm(&mut bridge, &mut module).unwrap();

    bridge.modules["sample_mod"]
        .print_to_file("test.ll")
        .unwrap();

    Ok(())
}

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
