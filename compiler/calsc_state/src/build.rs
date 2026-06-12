//! Building state definitions

use std::{collections::HashSet, path::PathBuf};

pub enum BuildTargetMode {
    Check,
    Remir,
    VendorIR,
    Object,
    Executable,
}

pub struct CompilerBuildState {
    pub(crate) files_to_build: HashSet<PathBuf>,
    pub out: PathBuf,
    pub target: BuildTargetMode,
}

impl CompilerBuildState {
    pub fn new(out: PathBuf, target: BuildTargetMode) -> Self {
        Self {
            files_to_build: HashSet::new(),
            out,
            target,
        }
    }

    pub fn append_to_build(&mut self, path: PathBuf) {
        self.files_to_build.insert(path);
    }
}

impl BuildTargetMode {
    pub fn requires_remir(&self) -> bool {
        match self {
            Self::Check => false,
            _ => true,
        }
    }

    pub fn requires_vendor_ir(&self) -> bool {
        match self {
            Self::Check | Self::Remir => false,
            _ => true,
        }
    }

    pub fn requires_object_files(&self) -> bool {
        match self {
            Self::Check | Self::Remir | Self::VendorIR => false,
            _ => true,
        }
    }

    pub fn requires_linking(&self) -> bool {
        match self {
            Self::Executable => true,
            _ => false,
        }
    }
}
