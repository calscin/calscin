//! Utilities related to position

use std::path::PathBuf;

/// A position within a file.
/// Is used to generate errors with positions
#[derive(Clone)]
pub struct FilePosition {
    /// The path of the file as a path
    pub file_path: PathBuf,

    /// The current line of the position
    pub line: usize,

    /// The column of the position
    pub column: usize,
}

impl FilePosition {
    /// Creates a new file position from the given arguments.
    ///
    /// # Example
    /// ```
    /// use std::path::PathBuf;
    /// use calsc_utils::pos::FilePosition;
    ///
    /// let pos: FilePosition = FilePosition::new(PathBuf::from("./test"), 1, 28);
    /// ```
    pub fn new(file_path: PathBuf, line: usize, column: usize) -> Self {
        Self {
            file_path,
            line,
            column,
        }
    }

    /// Clones the current `FilePosition` and adds the given amount of line to the line counter.
    /// This doesn't handle line / column breaks over the actual file
    ///
    /// # Example
    /// ```
    /// use std::path::PathBuf;
    /// use calsc_utils::pos::FilePosition;
    ///
    /// let pos: FilePosition = FilePosition::new(PathBuf::from("./test"), 1, 28);
    /// let pos2: FilePosition = pos.step_line(12);
    ///
    /// assert!(pos2.line == 13)
    /// ```
    #[inline]
    pub fn step_line(&self, gain_line: usize) -> Self {
        Self {
            file_path: self.file_path.clone(),
            line: self.line + gain_line,
            column: self.column,
        }
    }

    /// Adds 1 to the current line counter and resets the column counter for the given line.
    ///
    /// # Example
    /// ```
    /// use std::path::PathBuf;
    /// use calsc_utils::pos::FilePosition;
    ///
    /// let mut pos: FilePosition = FilePosition::new(PathBuf::from("./test"), 1, 28);
    /// pos.new_line();
    ///
    /// assert!(pos.line == 2);
    /// assert!(pos.column == 0);
    /// ```
    #[inline]
    pub fn new_line(&mut self) {
        self.line += 1;
        self.column = 0;
    }

    /// Clones the current `FilePosition` and adds the given amount of column to the column counter.
    /// This doesn't handle line breaks over the actual file
    ///
    /// # Example
    /// ```
    /// use std::path::PathBuf;
    /// use calsc_utils::pos::FilePosition;
    ///
    /// let pos: FilePosition = FilePosition::new(PathBuf::from("./test"), 1, 28);
    /// let pos2: FilePosition = pos.step_col(2);
    ///
    /// assert!(pos2.column == 30)
    /// ```
    #[inline]
    pub fn step_col(&self, gain_column: usize) -> Self {
        Self {
            file_path: self.file_path.clone(),
            line: self.line,
            column: self.column + gain_column,
        }
    }
}
