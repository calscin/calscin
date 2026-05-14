use calsc_diagnostics::{DiagResult, diags::errors::build_expected_error};
use calsc_lexer::toks::{Token, TokenKind};

/// Parses an list of elements in the AST of type `R` by using the given element parsing function.
///
/// **The provided parsing function should not post-increment** as it is already handled by the `parse_ast_list` function if `post_increments` is true.
/// If it is false, the `parse_ast_list` function will not post-increment after the provided parsing function.
///
/// This function post-increments after the closing token.
///
/// # Errors
/// The function will return an error **if `requires_at_least_one` is true and that the list is empty at the end.**
///
/// The function will return an error **if the element parsing function fails**
///
/// The function will return an error **if elements aren't properly seperated or if the parsing fails**
///
/// # Example
/// ```
/// use calsc_ast::parser::utils::parse_ast_list;
/// use calsc_lexer::lexer_tokenize;
/// use calsc_lexer::toks::{Token, TokenKind};
///
/// let tokens: Vec<Token> = lexer_tokenize("<123, 4536, 789>", "test.cal".to_string()).unwrap();
///
/// let mut ind: usize = 1; // Skip over the start of the list
///	let myList: Vec<i128> = parse_ast_list(&tokens, &mut ind, &mut |toks, ind| {
/// 	toks[*ind].expects_int_lit()
/// }, TokenKind::AngelBracketClose, true, true).unwrap();
///
///
/// ```
///
pub fn parse_ast_list<F, R>(
    tokens: &Vec<Token>,
    ind: &mut usize,
    func: &mut F,
    closing_point: TokenKind,
    requires_at_least_one: bool,
    post_increments: bool,
) -> DiagResult<Vec<R>>
where
    F: FnMut(&Vec<Token>, &mut usize) -> DiagResult<R>,
{
    let mut elements: Vec<R> = vec![];

    loop {
        if tokens[*ind].kind == closing_point {
            if elements.is_empty() && requires_at_least_one {
                println!("Empty");

                return Err(build_expected_error(
                    &"type parameter",
                    &tokens[*ind].kind,
                    &tokens[*ind],
                )
                .into());
            }

            *ind += 1; // closing point
            break;
        }

        let elem = func(tokens, ind)?;

        if post_increments {
            *ind += 1;
        }

        elements.push(elem);

        if tokens[*ind].kind == closing_point {
            *ind += 1; // closing point
            break;
        }

        tokens[*ind].expects(TokenKind::Comma)?;
        *ind += 1; // ,
    }

    Ok(elements)
}
