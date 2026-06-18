use std::path::PathBuf;

use crate::{
    path::ModulePath,
    tree::{
        ModuleTree,
        entry::{ModuleTreeEntry, TreeModule},
    },
};

pub trait ModuleTreeCollector {
    /// Collects the entries contained within the element if they meet the given condition. Also returns the paths to the entries
    fn collect_entries<F>(
        &self,
        f: &F,
        current_path: ModulePath,
        entries: &mut Vec<(ModuleTreeEntry, ModulePath)>,
    ) where
        F: Fn(&ModuleTreeEntry) -> bool;

    /// Collects the paths contained inside of the  module tree
    fn collect_paths(&self, vec: &mut Vec<PathBuf>);
}

impl ModuleTreeCollector for TreeModule {
    fn collect_paths(&self, vec: &mut Vec<PathBuf>) {
        if self.path.is_some() {
            vec.push(self.path.clone().unwrap())
        }

        for child in &self.children {
            child.1.collect_paths(vec);
        }
    }

    fn collect_entries<F>(
        &self,
        f: &F,
        current_path: ModulePath,
        entries: &mut Vec<(ModuleTreeEntry, ModulePath)>,
    ) where
        F: Fn(&ModuleTreeEntry) -> bool,
    {
        for child in &self.children {
            let mut path = current_path.clone();
            path.append_single_bit(child.0.clone());

            let entry = child.1;

            entry.collect_entries(f, path, entries);
        }
    }
}

impl ModuleTreeCollector for ModuleTreeEntry {
    fn collect_paths(&self, vec: &mut Vec<PathBuf>) {
        match self {
            Self::Module(module) => module.collect_paths(vec),
            _ => {}
        }
    }

    fn collect_entries<F>(
        &self,
        f: &F,
        current_path: ModulePath,
        entries: &mut Vec<(ModuleTreeEntry, ModulePath)>,
    ) where
        F: Fn(&ModuleTreeEntry) -> bool,
    {
        if f(self) {
            entries.push((self.clone(), current_path.clone()));
        }

        match self {
            Self::Module(module) => module.collect_entries(f, current_path, entries),
            _ => {}
        }
    }
}

impl ModuleTreeCollector for ModuleTree {
    fn collect_paths(&self, vec: &mut Vec<PathBuf>) {
        for entry in &self.entries {
            entry.1.collect_paths(vec);
        }
    }

    fn collect_entries<F>(
        &self,
        f: &F,
        current_path: ModulePath,
        entries: &mut Vec<(ModuleTreeEntry, ModulePath)>,
    ) where
        F: Fn(&ModuleTreeEntry) -> bool,
    {
        for entry in &self.entries {
            let mut path = current_path.clone();
            path.append_single_bit(entry.0.clone());

            let entry = entry.1;

            entry.collect_entries(f, path, entries);
        }
    }
}
