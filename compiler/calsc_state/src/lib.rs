//! The global state of the compiler. Mostly stores global configuration that can be available at all layers
//!

use std::{cell::RefCell, path::PathBuf};

use crate::build::{BuildTargetMode, CompilerBuildState};

pub mod build;

thread_local! {
    pub static GLOBAL_STATE: RefCell<CompilerGlobalState> = RefCell::new(CompilerGlobalState::new(None, BuildTargetMode::Check))
}

pub struct CompilerGlobalState {
    pub build: CompilerBuildState,
}

impl CompilerGlobalState {
    pub fn new(out: Option<PathBuf>, target: BuildTargetMode) -> Self {
        Self {
            build: CompilerBuildState::new(out, target),
        }
    }

    pub fn attach_build_config(&mut self, out: PathBuf, target: BuildTargetMode) {
        self.build.out = Some(out);
        self.build.target = target;
    }
}
