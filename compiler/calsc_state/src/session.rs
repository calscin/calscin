//! The new way of storing compiler session-global information.
//! This is favored instead of GlobalState since this allows for potentially parallel compilations

use calsc_tree_low::ctx::TreeLowCtx;
use calsc_typing::TypingInterner;

use crate::GlobalState;

/// Represents a compilation session from the compiler.
/// This is created once per compilation session and is shared accross module file builds for example.
/// This is passed onto the HIR and every layer that requires it
#[derive(Debug)]
pub struct CompilerSession<'session> {
    pub state: GlobalState,
    pub tree_lowered: Option<TreeLowCtx<'session>>,

    pub type_interner: TypingInterner,
}

impl<'session> CompilerSession<'session> {
    pub fn new() -> Self {
        Self {
            state: GlobalState::Pre,
            tree_lowered: None,
            type_interner: TypingInterner::new(),
        }
    }

    pub fn get_tree_lowered<'a>(&'a self) -> &'a TreeLowCtx<'session> {
        self.tree_lowered.as_ref().expect(&format!(
            "Lowered module tree is None in stage {:#?}!",
            self.state
        ))
    }

    pub fn get_tree_lowered_mut<'a: 'session>(&'a mut self) -> &'a mut TreeLowCtx<'session> {
        self.tree_lowered.as_mut().expect(&format!(
            "Lowered module tree is None in stage {:#?}!",
            self.state
        ))
    }
}

impl<'session> Default for CompilerSession<'session> {
    fn default() -> Self {
        Self::new()
    }
}
