//! Parsing for control elements (eg: if, for loops and more)

use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::{Token, TokenKind};

use crate::{nodes::ASTNode, parser::values::parse_ast_value};

pub mod for_loop;
pub mod ifelse;
pub mod loops;
pub mod while_loop;
