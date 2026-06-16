//! The file-based context of the HIR

use calsc_modules::path::ModulePath;
use calsc_state::GLOBAL_STATE;
use calsc_utils::hash::HashedString;

use crate::imports::LazyImportQueueElement;

pub struct HIRFileContext {
    pub current_module: ModulePath,
    pub lazy_imports: Vec<LazyImportQueueElement>,
}

impl HIRFileContext {
    pub fn new() -> Self {
        Self {
            current_module: ModulePath::new(
                GLOBAL_STATE.with_borrow(|state| state.package_name.clone()),
                vec![],
            ),

            lazy_imports: vec![],
        }
    }

    pub fn new_with_package(package: HashedString) -> Self {
        Self {
            current_module: ModulePath::new(package, vec![]),
            lazy_imports: vec![],
        }
    }

    pub fn advance_module(&mut self, path: HashedString) {
        self.current_module.path.push(path);
    }

    pub fn deadvance_module(&mut self) {
        self.current_module.path.pop();
    }
}
