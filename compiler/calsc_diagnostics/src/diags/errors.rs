//! The error declarations

use crate::declare_diagnostic;

// Is triggered whenever the Lexer cannot parse something (eg: cannot parse literals).
declare_diagnostic!(CANNOT_PARSE, 1);
