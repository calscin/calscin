//! The global state of the compiler. Mostly stores global configuration that can be available at all layers
//!

use std::{cell::RefCell, path::PathBuf};

use calsc_modules::tree::ModuleTree;
use calsc_utils::hash::HashedString;

use crate::build::{BuildTargetMode, CompilerBuildState};

pub mod build;

thread_local! {
    pub static GLOBAL_STATE: RefCell<CompilerGlobalState> = RefCell::new(CompilerGlobalState::new(None, BuildTargetMode::Check))
}

pub struct CompilerGlobalState {
    pub build: CompilerBuildState,
    pub package_name: HashedString,
    pub is_package_enabled: bool,
    pub module_tree: ModuleTree,
}

impl CompilerGlobalState {
    pub fn new(out: Option<PathBuf>, target: BuildTargetMode) -> Self {
        Self {
            build: CompilerBuildState::new(out, target, "".to_string()),
            package_name: "test_pkg".into(),
            is_package_enabled: false,
            module_tree: ModuleTree::new(),
        }
    }

    pub fn attach_build_config(&mut self, out: PathBuf, target: BuildTargetMode) {
        self.build.out = Some(out);
        self.build.target = target;
    }
}
