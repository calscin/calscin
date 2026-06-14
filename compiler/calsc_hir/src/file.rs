//! The file-based context of the HIR

use calsc_modules::path::ModulePath;
use calsc_state::GLOBAL_STATE;
use calsc_utils::hash::HashedString;

pub struct HIRFileContext {
    pub current_module: ModulePath,
}

impl HIRFileContext {
    pub fn new() -> Self {
        Self {
            current_module: ModulePath::new(
                GLOBAL_STATE.with_borrow(|state| state.package_name.clone()),
                vec![],
            ),
        }
    }

    pub fn advance_module(&mut self, path: HashedString) {
        self.current_module.path.push(path);
    }
}
