//! Building state definitions

use std::{collections::HashSet, path::PathBuf};

#[derive(Clone)]
pub enum BuildTargetMode {
    Check,
    Remir,
    VendorIR,
    Object,
    Executable,
}

/// Represents the current state for building inside of the Calscin compiler.
/// This uses a file consuming architecture which allows to efficiently handle
/// module dependencies at compile time and build files that are only discovered during the compile time
pub struct CompilerBuildState {
    pub(crate) files_to_build: HashSet<PathBuf>,
    pub(crate) additional_files_built: HashSet<PathBuf>,
    pub origin_file_to_build: Option<PathBuf>,
    pub out: Option<PathBuf>,
    pub target: BuildTargetMode,
    pub linker: String,
    pub use_pie: bool,
}

impl CompilerBuildState {
    /// Creates a new [`CompilerBuildState`] based on the given out path and target mode
    pub fn new(out: Option<PathBuf>, target: BuildTargetMode, linker: String) -> Self {
        Self {
            files_to_build: HashSet::new(),
            additional_files_built: HashSet::new(),
            origin_file_to_build: None,
            out,
            target,
            linker,
            use_pie: false,
        }
    }

    /// Appends the file to the building queue
    pub fn append_to_build(&mut self, path: PathBuf) {
        if self.additional_files_built.contains(&path) {
            return;
        }

        self.additional_files_built.insert(path.clone());
        self.files_to_build.insert(path);
    }

    /// Consumes the file building queue and empties it.
    /// This should be used to compile these files
    /// A correct way to check if the compilation is ended is to see when the consume result is empty
    pub fn consume_files(&mut self) -> Vec<PathBuf> {
        if self.files_to_build.is_empty() {
            return vec![];
        }

        let files: Vec<_> = self.files_to_build.iter().map(|f| f.clone()).collect();

        self.files_to_build.clear();
        files
    }

    /// Gets the remaining amount of files in the queue.
    /// Warn: this is not the total amount of files but the current amount
    pub fn get_remaining_files(&self) -> usize {
        self.files_to_build.len()
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
