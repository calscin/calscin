#![deny(unsafe_code)]

use std::{ffi::OsStr, fs, path::PathBuf, process::Command};

use calsc_ast::parser::ctx::parse_ast_whole;
use calsc_diagnostics::{
    container::dump_and_stop_if_errors, diags::errors::import_wrong_entry_point,
    file::FileDiagnosticPos, panics::PanicDiagnosticSource, result::CalscinResult,
};
use calsc_hir::{HIRContext, file::HIRFileContext};
use calsc_hir_lowering::{stage1::import_everything_inside_module, stage2::lower_hir_stage_2};
use calsc_lexer::lexer_tokenize;
use calsc_remir_lowering::compile_file;
use calsc_state::{GLOBAL_STATE, build::BuildTargetMode, session::CompilerSession};
use calsc_tree_build::{build_module_tree, ctx::TreeBuildingCtx};
use calsc_tree_low::{ctx::TreeLowCtx, lower::lower_everything};

pub fn setup_build_state(
    out: PathBuf,
    target: BuildTargetMode,
    initial_file: PathBuf,
    linker: String,
    use_pie: bool,
) {
    GLOBAL_STATE.with_borrow_mut(|state| {
        state.attach_build_config(out, target);
        state.build.linker = linker;
        state.build.use_pie = use_pie;

        state.build.append_to_build(initial_file.clone());
        state.build.origin_file_to_build = Some(initial_file);
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

pub(crate) fn get_linker() -> String {
    GLOBAL_STATE.with_borrow(|state| state.build.linker.clone())
}

pub fn build() {
    let mut out_files: Vec<PathBuf> = vec![];

    let mut session = CompilerSession::new();

    if GLOBAL_STATE.with_borrow(|f| {
        f.is_package_enabled
            && f.build.origin_file_to_build.clone().unwrap().file_name()
                != Some(OsStr::new("module.cal"))
    }) {
        import_wrong_entry_point(&FileDiagnosticPos::new(
            GLOBAL_STATE.with_borrow(|f| f.build.origin_file_to_build.clone().unwrap()),
        ));

        dump_and_stop_if_errors();
    }

    // Building global module tree
    let path = GLOBAL_STATE.with_borrow(|f| f.build.origin_file_to_build.clone().unwrap());

    let mut ctx = TreeBuildingCtx::new(
        GLOBAL_STATE.with_borrow(|f| f.package_name.clone()),
        &PanicDiagnosticSource(),
        GLOBAL_STATE.with_borrow(|f| f.is_package_enabled),
    );

    build_module_tree(path, &mut ctx).unwrap_cleanly();

    let used_files = session.get_tree_lowered().build_ctx.tree.used_files.clone();

    let mut typing_interner = std::mem::take(&mut session.type_interner);

    let mut lowered_ctx = TreeLowCtx::new(ctx, &mut typing_interner);

    lower_everything(&mut lowered_ctx).unwrap_cleanly();

    session.tree_lowered = Some(lowered_ctx.data);
    session.type_interner = typing_interner;

    for file in used_files {
        let out_file = build_file(file, &mut session);

        if let Some(path) = out_file {
            out_files.push(path);
        }
    }

    if get_target_type().requires_linking() {
        let mut command = Command::new(get_linker());

        for file in &out_files {
            command.arg(file.to_str().unwrap());
        }

        command.arg(format!("-o{}", get_file_output().to_str().unwrap()));

        let output = command.output().unwrap();

        println!("{}", String::from_utf8_lossy(&output.stderr));

        // Cleaning

        for out in out_files {
            fs::remove_file(out).unwrap();
        }
    }
}

pub fn build_file<'session>(
    file: PathBuf,
    session: &'session mut CompilerSession,
) -> Option<PathBuf> {
    let target = get_target_type(); // Avoid borrows
    let out_destination = get_file_output(); // Avoid borrows

    let contents = match fs::read_to_string(file.clone()) {
        Ok(v) => v,
        Err(e) => panic!("IO error: {e}"),
    };

    let lexer = lexer_tokenize(&contents, file.to_str().unwrap().to_string());
    dump_and_stop_if_errors();

    let ast_ctx = parse_ast_whole(&lexer.unwrap());
    dump_and_stop_if_errors();

    let ast_ctx = ast_ctx.unwrap();

    let mut hir_ctx = HIRContext::new(session);
    let mut file_ctx = HIRFileContext::new(file.clone());

    let _ = import_everything_inside_module(
        file_ctx.current_module.clone(),
        &file,
        &mut hir_ctx,
        &FileDiagnosticPos::new(file.clone()),
    );
    dump_and_stop_if_errors();

    let _ = lower_hir_stage_2(ast_ctx, &mut hir_ctx, &mut file_ctx);
    dump_and_stop_if_errors();

    if !target.requires_remir() {
        return None;
    }

    let mut out_file = match target {
        BuildTargetMode::Remir => file.with_extension("remir"),
        BuildTargetMode::Object => file.with_extension("o"),
        BuildTargetMode::VendorIR => file.with_extension("ll"),
        BuildTargetMode::Executable => file.with_extension("o"),

        _ => panic!(),
    };

    if out_destination.is_dir() {
        out_file = out_destination.join(out_file);
    }

    let _ = compile_file(
        hir_ctx,
        out_file.clone(),
        file.clone(),
        file.file_name().unwrap().to_str().unwrap().to_string(),
        target.clone(),
    );
    dump_and_stop_if_errors();

    Some(out_file)
}
