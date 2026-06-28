#![deny(unsafe_code)]

use std::path::PathBuf;

use calsc_diagnostics::{DiagPossible, PosDiagnosticSource, file::FileDiagnosticPos};
use calsc_hir::HIRContext;
use calsc_state::{GLOBAL_STATE, build::BuildTargetMode};
use calsc_utils::pos::FilePosition;
use remir::module::Module;
use remir_llvm::{LLVMBridge, build_llvm, compile_llvm};

use crate::{
    funcs::{lower_hir_function_decl, lower_hir_function_decl_none},
    mono::Monomorphizer,
    result::CalscinRemirResult,
};

pub mod body;
pub mod control;
pub mod funcs;
pub mod indexes;
pub mod mono;
pub mod range;
pub mod reads;
pub mod result;
pub mod types;
pub mod values;
pub mod vars;
pub mod writes;

pub fn lower_hir_context(mut ctx: HIRContext, module: &mut Module) -> DiagPossible {
    let dummy_pos = PosDiagnosticSource::new(FilePosition::default(), FilePosition::default()); // This is okay since we are sure that as_function won't fail

    // First round: registering functions
    let keys: Vec<_> = ctx.scope.key_to_ind.keys().map(|f| f.clone()).collect();

    for key in keys {
        let entry = ctx
            .scope
            .get_entry_no_visibility(key.clone(), &dummy_pos)?
            .clone();

        if entry.is_function() {
            let func = entry.as_function(&dummy_pos)?;

            if !func.type_parameters.is_empty() {
                Monomorphizer::monomorph_function_definitions(module, func, &mut ctx)?;
            } else {
                lower_hir_function_decl_none(
                    key.clone(),
                    func.arguments.iter().map(|f| f.1.clone()).collect(),
                    func.return_type.clone(),
                    func.is_main_function,
                    module,
                    func.triple_dot_position,
                    &mut ctx,
                )?;
            }
        }
    }

    // Second round: lowering bodies
    let keys_to_ind_clone = ctx.scope.key_to_ind.clone();

    for key in keys_to_ind_clone.keys() {
        let entry = ctx
            .scope
            .get_entry_no_visibility(key.clone(), &dummy_pos)?
            .clone();

        if entry.is_function() {
            let func = entry.as_function(&dummy_pos)?;

            if func.impl_node.is_some() {
                if !func.type_parameters.is_empty() {
                    Monomorphizer::monomorph_function_declarations(
                        func.impl_node.clone().unwrap(),
                        &mut ctx,
                        module,
                    )?;
                } else {
                    lower_hir_function_decl(func.impl_node.clone().unwrap(), &mut ctx, module)?;
                }
            }
        }
    }

    Ok(())
}

pub fn compile_file(
    ctx: HIRContext,
    out: PathBuf,
    file_in: PathBuf,
    module_name: String,
    target: BuildTargetMode,
) -> DiagPossible {
    let mut module = Module::new(module_name.clone());
    let file_source = FileDiagnosticPos::new(file_in);

    lower_hir_context(ctx, &mut module)?;

    if !target.requires_vendor_ir() {
        module.save_to_file(out).unwrap();

        return Ok(());
    }

    let mut bridge = LLVMBridge::new();

    if !target.requires_object_files() {
        build_llvm(&mut bridge, &mut module)?;

        bridge.modules[&module_name].print_to_file(out).unwrap();

        return Ok(());
    }

    compile_llvm(
        &mut bridge,
        &mut module,
        remir::OptimizationLevel::Default,
        out,
        GLOBAL_STATE.with_borrow(|state| state.build.use_pie),
    )
    .convert_source(&file_source)?;

    Ok(())
}
