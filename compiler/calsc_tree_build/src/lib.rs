//! This layer runs right after the *AST* and is mostly made to generate a module tree of the package, containing every used module in a tree structure.
//! Each module containing the following information:
//! - *Elements*: the children / symbols of the module. Can also contain child modules
//! - *Imports*: the imports used inside of the module is a table of `imported element name / path -> true full path inside of the package`
//! - *File*: the file that made this module / the file origin of the module.
//!
//! Furthermore the root of the module tree itself will store the following information:
//! - Module tree
//! - `Path -> Module Entry` resolved cache
//! - Set of files that are used for every discovered modules (`HashSet<PathBuf>`)
//!
//!
//! We also associate every entry with the related nodes, for example every type tree entry should be related to the AST node that caused it to be added / to exist.
