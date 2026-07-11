//! The global state of the compiler. Mostly stores global configuration that can be available at all layers
//!

use std::{cell::RefCell, path::PathBuf};

use calsc_tree_low::ctx::TreeLowCtx;
use calsc_utils::hash::HashedString;

use crate::build::{BuildTargetMode, CompilerBuildState};

pub mod build;
pub mod interners;
pub mod session;

thread_local! {
    pub static GLOBAL_STATE: RefCell<CompilerGlobalState<'static>> = RefCell::new(CompilerGlobalState::new(None, BuildTargetMode::Check))
}

#[derive(Clone, Debug)]
pub enum GlobalState {
    Pre,
    ModuleLow,
    HIR,
}

pub struct CompilerGlobalState<'session> {
    pub build: CompilerBuildState,
    pub package_name: HashedString,
    pub is_package_enabled: bool,

    pub global_state: GlobalState,

    /// Available after GlobalState::ModuleLow
    pub tree_lowered: Option<TreeLowCtx<'session>>, // 'static is fine because CompilerGlobalState lives infinitely
}

impl<'session> CompilerGlobalState<'session> {
    pub fn new(out: Option<PathBuf>, target: BuildTargetMode) -> Self {
        Self {
            build: CompilerBuildState::new(out, target, "".to_string()),
            package_name: "test_pkg".into(),
            global_state: GlobalState::Pre,
            is_package_enabled: false,
            tree_lowered: Option::None,
        }
    }

    pub fn attach_build_config(&mut self, out: PathBuf, target: BuildTargetMode) {
        self.build.out = Some(out);
        self.build.target = target;
    }

    pub fn get_tree_lowered<'a>(&'a self) -> &'a TreeLowCtx<'a> {
        self.tree_lowered.as_ref().expect(&format!(
            "Lowered module tree is None in stage {:#?}!",
            self.global_state
        ))
    }

    pub fn get_tree_lowered_mut<'a: 'session>(&'a mut self) -> &'a mut TreeLowCtx<'a> {
        self.tree_lowered.as_mut().expect(&format!(
            "Lowered module tree is None in stage {:#?}!",
            self.global_state
        ))
    }
}
