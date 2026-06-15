//! Parsing of if statements & else statements

use crate::{ASTContext, refs::ASTArenaReference};
use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::{Token, TokenKind};

use crate::{
    ifs::IfStatementBranch,
    nodes::{ASTNode, ASTNodeKind},
    parser::forms::{parse_ast_body_form, parse_ast_condition_form},
};

fn parse_if_member_statement(
    tokens: &Vec<Token>,
    ind: &mut usize,
    ctx: &mut ASTContext,
) -> DiagResult<IfStatementBranch> {
    *ind += 1; // if

    let condition = parse_ast_condition_form(tokens, ind, ctx)?; // Auto increments
    let body = parse_ast_body_form(tokens, ind, ctx)?; // Auto increments

    Ok(IfStatementBranch::If { condition, body })
}

#[inline(always)]
fn parse_if_else_member_statement(
    tokens: &Vec<Token>,
    ind: &mut usize,
    ctx: &mut ASTContext,
) -> DiagResult<IfStatementBranch> {
    *ind += 2; // else & if

    let condition = parse_ast_condition_form(tokens, ind, ctx)?; // Auto increments
    let body = parse_ast_body_form(tokens, ind, ctx)?; // Auto increments

    Ok(IfStatementBranch::IfElse { condition, body })
}

fn parse_else_member_statement(
    tokens: &Vec<Token>,
    ind: &mut usize,
    has_met_else: &mut bool,
    ctx: &mut ASTContext,
) -> DiagResult<IfStatementBranch> {
    if tokens[*ind + 1].kind == TokenKind::If {
        return parse_if_else_member_statement(tokens, ind, ctx);
    } else {
        *ind += 1; // else

        *has_met_else = true;

        let body = parse_ast_body_form(tokens, ind, ctx)?;

        return Ok(IfStatementBranch::Else { body });
    }
}

#[inline(always)]
pub fn parse_ast_if_statement(
    tokens: &Vec<Token>,
    ind: &mut usize,
    ctx: &mut ASTContext,
) -> DiagResult<ASTArenaReference> {
    let start = tokens[*ind].start.clone();

    let mut statements: Vec<IfStatementBranch> = vec![];

    let if_statement = parse_if_member_statement(tokens, ind, ctx)?; // Auto increments
    let mut has_met_else = false;

    statements.push(if_statement);

    while tokens[*ind].kind == TokenKind::Else {
        let statement = parse_else_member_statement(tokens, ind, &mut has_met_else, ctx)?; // Auto increments

        statements.push(statement);

        if has_met_else {
            break;
        }
    }

    let end = tokens[*ind - 1].end.clone();

    let node = ASTNode::new(
        ASTNodeKind::IfStatement {
            branches: statements,
        },
        start,
        end,
    );

    Ok(node.push(ctx))
}
