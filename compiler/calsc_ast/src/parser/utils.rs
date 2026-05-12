use calsc_diagnostics::{DiagResult, diags::errors::build_expected_error};
use calsc_lexer::toks::{Token, TokenKind};

/// Parses an list of elements in the AST of type `R` by using the given element parsing function.
///
/// # Errors
/// The function will return an error **if `requires_at_least_one` is true and that the list is empty at the end.**
///
/// The function will return an error **if the element parsing function fails**
///
/// The function willr return an error **if elements aren't properly seperated or if the parsing fails**
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
///	let myList: Vec<i128> = parse_ast_list(&tokens, &mut ind, |toks, ind| {
/// 	toks[*ind].expects_int_lit()
/// }, TokenKind::AngelBracketClose, true).unwrap();
///
///
/// ```
///
pub fn parse_ast_list<F, R>(
    tokens: &Vec<Token>,
    ind: &mut usize,
    func: F,
    closing_point: TokenKind,
    requires_at_least_one: bool,
) -> DiagResult<Vec<R>>
where
    F: Fn(&Vec<Token>, &mut usize) -> DiagResult<R>,
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

        println!("Pre {:#?}", tokens[*ind].kind);
        let elem = func(tokens, ind)?;
        *ind += 1;
        println!("Post");

        elements.push(elem);

        if tokens[*ind].kind == closing_point {
            *ind += 1; // closing point
            break;
        }

        println!("Pre comma {:#?}", tokens[*ind].kind);

        println!("Expects comma");
        tokens[*ind].expects(TokenKind::Comma)?;
        println!("Post expects");
        *ind += 1; // ,
    }

    Ok(elements)
}
