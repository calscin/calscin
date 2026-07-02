use calsc_diagnostics::{DiagResult, diags::errors::build_unexpected_token_error};
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::math::{MathOperation, MathOperator};

#[inline(always)]
pub fn parse_ast_math_operator(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<MathOperator> {
    let mut fast = false;
    let mut assigns = false;

    if tokens[*ind].kind == TokenKind::Tilde {
        *ind += 1;
        fast = true;
    }

    let operator = match &tokens[*ind].kind {
        TokenKind::Plus => {
            if tokens[*ind + 1].kind == TokenKind::Plus {
                *ind += 1; // first +

                MathOperation::And
            } else {
                MathOperation::Add
            }
        }

        TokenKind::Minus => {
            if tokens[*ind + 1].kind == TokenKind::Minus {
                *ind += 1; // first -

                MathOperation::Or
            } else {
                MathOperation::Sub
            }
        }

        TokenKind::Star => {
            if tokens[*ind + 1].kind == TokenKind::Star {
                *ind += 1; // first *

                MathOperation::ShiftLeft
            } else {
                MathOperation::Mul
            }
        }

        TokenKind::Slash => {
            if tokens[*ind + 1].kind == TokenKind::Slash {
                *ind += 1; // first /

                MathOperation::ShiftRight
            } else {
                MathOperation::Div
            }
        }
        TokenKind::BackSlash => MathOperation::Mod,

        TokenKind::Bang => {
            *ind += 1; // first !

            tokens[*ind].expects(TokenKind::Bang)?;
            MathOperation::Xor
        }

        _ => return Err(build_unexpected_token_error(&tokens[*ind].kind, &tokens[*ind]).into()),
    };

    *ind += 1; // post increment for every second character / single character operation

    if tokens[*ind].kind == TokenKind::Equal {
        assigns = true;
        *ind += 1; // =
    }

    Ok(MathOperator::new(operator, fast, assigns))
}
