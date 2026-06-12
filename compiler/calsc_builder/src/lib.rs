use std::{fs, hint::unreachable_unchecked, path::PathBuf};

use calsc_ast::{AST_CONTEXT, parser::ctx::parse_ast_whole};
use calsc_diagnostics::container::dump_and_stop_if_errors;
use calsc_hir::HIR_CONTEXT;
use calsc_hir_lowering::{stage1::lower_hir_stage_1, stage2::lower_hir_stage_2};
use calsc_lexer::lexer_tokenize;
use calsc_remir_lowering::compile_file;
use calsc_state::{GLOBAL_STATE, build::BuildTargetMode};

pub fn setup_build_state(
    out: PathBuf,
    target: BuildTargetMode,
    initial_files: Vec<PathBuf>,
    linker: String,
) {
    GLOBAL_STATE.with_borrow_mut(|state| {
        state.attach_build_config(out, target);
        state.build.linker = linker;

        for file in initial_files {
            state.build.append_to_build(file);
        }
    })
}

pub fn get_remaining_files_to_build() -> usize {
    GLOBAL_STATE.with_borrow(|f| f.build.get_remaining_files())
}

pub(crate) fn get_target_type() -> BuildTargetMode {
    GLOBAL_STATE.with_borrow(|state| state.build.target.clone())
}

pub(crate) fn get_file_output() -> PathBuf {
    GLOBAL_STATE.with_borrow(|state| state.build.out.clone().unwrap())
}

pub(crate) fn consume_build_files(out_files: &mut Vec<PathBuf>) {
    let files = GLOBAL_STATE.with_borrow_mut(|state| state.build.consume_files());

    if files.is_empty() {
        return;
    }

    for file in files {
        let res = build_file(file);

        match res {
            Some(v) => out_files.push(v),
            None => continue,
        };
    }
}

pub fn build_file(file: PathBuf) -> Option<PathBuf> {
    let target = get_target_type(); // Avoid borrows
    let out_destination = get_file_output(); // Avoid borrows

    let contents = match fs::read_to_string(file.clone()) {
        Ok(v) => v,
        Err(e) => panic!("IO error: {e}"),
    };

    let lexer = lexer_tokenize(&contents, file.to_str().unwrap().to_string());
    dump_and_stop_if_errors();

    let _ = parse_ast_whole(&lexer.unwrap());
    dump_and_stop_if_errors();

    let ast_context = AST_CONTEXT.with(|ctx| ctx.borrow().clone()); // We keep it for AST refs

    let _ = lower_hir_stage_1(ast_context.clone());
    dump_and_stop_if_errors();

    let _ = lower_hir_stage_2(ast_context);
    dump_and_stop_if_errors();

    if !target.requires_remir() {
        return None;
    }

    let ctx_context = HIR_CONTEXT.with(|ctx| ctx.borrow().clone()); // We keep this for HIR refs

    let mut out_file = match target {
        BuildTargetMode::Remir => file.with_extension("remir"),
        BuildTargetMode::Object => file.with_extension("o"),
        BuildTargetMode::VendorIR => file.with_extension("ll"),
        BuildTargetMode::Executable => file.with_extension(""), // TODO: add windows support

        _ => unsafe { unreachable_unchecked() },
    };

    if out_destination.is_dir() {
        out_file = out_destination.join(out_file);
    }

    let _ = compile_file(
        ctx_context,
        out_file.clone(),
        file.file_name().unwrap().to_str().unwrap().to_string(),
        target.clone(),
    );
    dump_and_stop_if_errors();

    Some(out_file)
}
