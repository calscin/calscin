//! The new way of storing compiler session-global information.
//! This is favored instead of GlobalState since this allows for potentially parallel compilations

use calsc_tree_low::ctx::TreeLowCtx;

use crate::GlobalState;

/// Represents a compilation session from the compiler.
/// This is created once per compilation session and is shared accross module file builds for example.
/// This is passed onto the HIR and every layer that requires it
#[derive(Debug)]
pub struct CompilerSession {
    pub state: GlobalState,
    pub tree_lowered: Option<TreeLowCtx>,
}

impl CompilerSession {
    pub fn new() -> Self {
        Self {
            state: GlobalState::Pre,
            tree_lowered: None,
        }
    }

    pub fn get_tree_lowered<'a>(&'a self) -> &'a TreeLowCtx {
        self.tree_lowered.as_ref().expect(&format!(
            "Lowered module tree is None in stage {:#?}!",
            self.state
        ))
    }

    pub fn get_tree_lowered_mut<'a>(&'a mut self) -> &'a mut TreeLowCtx {
        self.tree_lowered.as_mut().expect(&format!(
            "Lowered module tree is None in stage {:#?}!",
            self.state
        ))
    }
}
