use crate::tree::{
    ModuleTree,
    entry::{ModuleTreeEntry, TreeModule},
};

pub trait TreeCleanable {
    fn is_clean(&self) -> bool;
    fn clean(&mut self);
}

impl TreeCleanable for TreeModule {
    fn clean(&mut self) {
        let mut to_delete = vec![];

        for child in &mut self.children {
            let entry = child.1;

            if entry.is_clean() {
                entry.clean();
            } else {
                to_delete.push(child.0.clone());
            }
        }

        for delete in to_delete {
            self.children.remove(&delete);
        }
    }

    fn is_clean(&self) -> bool {
        self.imported
    }
}

impl TreeCleanable for ModuleTreeEntry {
    fn clean(&mut self) {
        match self {
            Self::Module(module) => module.clean(),
            _ => {}
        }
    }

    fn is_clean(&self) -> bool {
        match self {
            Self::Module(module) => module.is_clean(),
            _ => true,
        }
    }
}

impl TreeCleanable for ModuleTree {
    fn is_clean(&self) -> bool {
        true
    }

    fn clean(&mut self) {
        let mut to_delete = vec![];

        for child in &mut self.entries {
            let entry = child.1;

            if entry.is_clean() {
                println!("Entry '{}' is clean", child.0);
                entry.clean();
            } else {
                println!("Entry '{}' is unclean", child.0);

                to_delete.push(child.0.clone());
            }
        }

        for delete in to_delete {
            self.entries.remove(&delete);
        }
    }
}
