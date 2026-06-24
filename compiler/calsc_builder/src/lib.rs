#![deny(unsafe_code)]

use std::{fs, path::PathBuf, process::Command};

use calsc_ast::parser::ctx::parse_ast_whole;
use calsc_diagnostics::{container::dump_and_stop_if_errors, result::CalscinResult};
use calsc_hir::{HIRContext, file::HIRFileContext};
use calsc_hir_lowering::{
    modules::build_module_tree, modules_lower::lower_types_from_stage_0, stage1::lower_hir_stage_1,
    stage2::lower_hir_stage_2,
};
use calsc_lexer::lexer_tokenize;
use calsc_modules::{
    path::ModulePath,
    tree::{clean::TreeCleanable, collect::ModuleTreeCollector},
};
use calsc_remir_lowering::compile_file;
use calsc_state::{GLOBAL_STATE, build::BuildTargetMode};

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

    // Building global module tree
    if GLOBAL_STATE.with_borrow(|state| state.is_package_enabled) {
        let module_tree = build_module_tree(
            GLOBAL_STATE.with_borrow(|f| f.build.origin_file_to_build.clone().unwrap()),
        );

        dump_and_stop_if_errors();

        let mut module_tree = module_tree.unwrap();

        module_tree.clean();

        let mut entries = vec![];

        module_tree.collect_entries(
            &|entry| entry.is_type(),
            ModulePath::new("".into(), vec![]),
            &mut entries,
        );

        lower_types_from_stage_0(&module_tree).unwrap_cleanly();

        GLOBAL_STATE.with_borrow_mut(|state| state.module_tree = module_tree);
    }

    loop {
        if !consume_build_files(&mut out_files) {
            break;
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

pub(crate) fn consume_build_files(out_files: &mut Vec<PathBuf>) -> bool {
    let files = GLOBAL_STATE.with_borrow_mut(|state| state.build.consume_files());

    if files.is_empty() {
        return false;
    }

    for file in files {
        let res = build_file(file);

        match res {
            Some(v) => out_files.push(v),
            None => continue,
        };
    }

    true
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

    let ast_ctx = parse_ast_whole(&lexer.unwrap());
    dump_and_stop_if_errors();

    let ast_ctx = ast_ctx.unwrap();

    let mut hir_ctx = HIRContext::new();
    let mut file_ctx = HIRFileContext::new();

    let _ = lower_hir_stage_1(ast_ctx.clone(), &mut hir_ctx, &mut file_ctx);
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
