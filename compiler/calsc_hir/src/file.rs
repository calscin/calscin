//! The file-based context of the HIR

use std::path::PathBuf;

use calsc_modules::path::ModulePath;
use calsc_state::GLOBAL_STATE;
use calsc_utils::{hash::HashedString, path::to_absolute_path};

use crate::imports::LazyImportQueueElement;

pub struct HIRFileContext {
    pub current_module: ModulePath,
    pub lazy_imports: Vec<LazyImportQueueElement>,
}

impl HIRFileContext {
    pub fn new(mut buff: PathBuf) -> Self {
        let mut dir =
            GLOBAL_STATE.with_borrow_mut(|state| state.build.origin_file_to_build.clone().unwrap());

        dir = to_absolute_path(dir.parent().unwrap().to_path_buf()).unwrap();
        buff = to_absolute_path(buff).unwrap();

        let path = buff.strip_prefix(dir).unwrap();

        let mut module_path = ModulePath::new(
            GLOBAL_STATE.with_borrow(|state| state.package_name.clone()),
            vec![],
        );

        for path in path.iter() {
            let path = path.to_str().unwrap().to_string();

            let path: HashedString = path.split(".").collect::<Vec<_>>()[0].into();

            if path != "module".into() {
                module_path.append_single_bit(path);
            }
        }

        Self {
            current_module: module_path,
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
