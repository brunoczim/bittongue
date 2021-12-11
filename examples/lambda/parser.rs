//! Parser functions. More specifically, only [`parse_expr`] should be used
//! outside this module as a general-purpose lambda calculus parser.

pub mod error;

use crate::{
    ast::{Expr, ExprKind},
    lexer::Lexer,
    token::TokenKind,
};
use bittongue::{
    diagnostic::Diagnostics,
    lexer::{Token, TokenStream},
};
use error::{MismatchedToken, UnmatchedCloseParen, UnmatchedOpenParen};
use TokenKind::*;

/// Dummy error for when parser cannot produce something. Might not be created
/// even if parse errors were found.
#[derive(Debug, Clone, Copy, Default)]
pub struct ParseError;

/// Parses a whole expression.
pub fn parse_expr(
    token_stream: &mut TokenStream<Lexer>,
    diagnostics: &mut Diagnostics,
) -> Result<Expr, ParseError> {
    parse_with_end(Eof, token_stream, diagnostics)
}

/// Expects a set of tokens. If the current token is not in the set, an error is
/// raised.
fn expect_token(
    expected: &[TokenKind],
    token_stream: &mut TokenStream<Lexer>,
    diagnostics: &mut Diagnostics,
) -> Result<Token<TokenKind>, ParseError> {
    let current = token_stream.current().map(Clone::clone);
    match current {
        Ok(token) => {
            if expected.contains(&token.kind) {
                token_stream.next(diagnostics);
                Ok(token)
            } else {
                diagnostics.raise(MismatchedToken {
                    expected: expected.to_vec(),
                    found: token,
                });
                Err(ParseError)
            }
        },
        Err(_) => {
            token_stream.next(diagnostics);
            Err(ParseError)
        },
    }
}

/// Parses a syntactical lambda.
fn parse_lambda(
    end: TokenKind,
    token_stream: &mut TokenStream<Lexer>,
    diagnostics: &mut Diagnostics,
) -> Result<Expr, ParseError> {
    let lambda_token = match token_stream.current() {
        Ok(token) => Ok(token.clone()),
        Err(_) => Err(ParseError),
    };
    token_stream.next(diagnostics);
    let parameter_result = expect_token(&[Ident], token_stream, diagnostics);
    let _ = expect_token(&[Dot], token_stream, diagnostics);
    let body = parse_with_end(end, token_stream, diagnostics)?;
    let parameter = parameter_result?;
    let full_span = match lambda_token {
        Ok(lambda) => lambda.span.join(&body.span),
        Err(_) => parameter.span.join(&body.span),
    };

    Ok(Expr {
        span: full_span,
        kind: ExprKind::Lambda {
            parameter: parameter.span,
            body: Box::new(body),
        },
    })
}

/// Parses an expression surrounded by parenthesis.
fn parse_parenthesized(
    open_paren_token: Token<TokenKind>,
    token_stream: &mut TokenStream<Lexer>,
    diagnostics: &mut Diagnostics,
) -> Result<Expr, ParseError> {
    token_stream.next(diagnostics);
    let mut output_expr =
        parse_with_end(CloseParen, token_stream, diagnostics)?;
    output_expr.span = output_expr.span.join(&open_paren_token.span);

    match token_stream.current() {
        Ok(token) if token.kind == CloseParen => {
            output_expr.span = output_expr.span.join(&token.span);
            token_stream.next(diagnostics);
        },
        _ => {
            let error = UnmatchedOpenParen { span: open_paren_token.span };
            diagnostics.raise(error);
        },
    }

    Ok(output_expr)
}

/// Parses an expression given the token kind that finishes it.
fn parse_with_end(
    end: TokenKind,
    token_stream: &mut TokenStream<Lexer>,
    diagnostics: &mut Diagnostics,
) -> Result<Expr, ParseError> {
    // Working expression. Initially absent. If still absent by the end of the
    // function, an error is raised.
    let mut curr_expr = None;
    // Whether this expression had a non-eof token.
    let mut had_child_token = false;

    loop {
        match token_stream.current() {
            Ok(token) => match token.kind {
                // Equals to the end? Done.
                kind if end == kind => break,

                // Lambda? Delegate to parse_lambda and stack the expressions.
                Lambda => {
                    if let Ok(lambda) =
                        parse_lambda(end, token_stream, diagnostics)
                    {
                        curr_expr = Some(stack_exprs(curr_expr, lambda));
                    }
                },

                // Identifier? Simply create a variable.
                Ident => {
                    let next_expr = Expr {
                        span: token.span.clone(),
                        kind: ExprKind::Variable(token.span.clone()),
                    };
                    curr_expr = Some(stack_exprs(curr_expr, next_expr));
                    token_stream.next(diagnostics);
                },

                // Opening parenthesis? Simply delegate to parse_parenthesized
                // and stack the expressions.
                OpenParen => {
                    if let Ok(next_expr) = parse_parenthesized(
                        token.clone(),
                        token_stream,
                        diagnostics,
                    ) {
                        curr_expr = Some(stack_exprs(curr_expr, next_expr));
                    }
                },

                // End-Of-Input?? Stop without errors, they will be produced
                // somewhereelse (given that EOF was not the expected finishing
                // token).
                Eof => break,

                // Closing parenthesis (and not expected to finish the
                // expression)? Issue an unmatched closing
                // parenthesis. If it is not the final token of this expression,
                // there is no valid opening parenthesis to match it.
                CloseParen => {
                    let error =
                        UnmatchedCloseParen { span: token.span.clone() };
                    diagnostics.raise(error);
                    token_stream.next(diagnostics);
                },

                // Otherwise, e.g. dot, the token was not expected.
                _ => {
                    let error = MismatchedToken {
                        expected: vec![Lambda, Ident, OpenParen, end],
                        found: token.clone(),
                    };
                    diagnostics.raise(error);
                    token_stream.next(diagnostics);
                },
            },

            // The lexer already issued an error here.
            Err(_) => {
                token_stream.next(diagnostics);
            },
        }
        had_child_token = true;
    }

    match curr_expr {
        // We have an expression? Good, return it even if there are errors.
        Some(expr) => Ok(expr),
        // No expression? Probably an error.
        None => {
            // Only issue the error if EOF was directly found, without any other
            // token kind in the way.
            if !had_child_token {
                let error = MismatchedToken {
                    expected: vec![Lambda, Ident, OpenParen],
                    found: token_stream.current().unwrap().clone(),
                };
                diagnostics.raise(error);
            }
            Err(ParseError)
        },
    }
}

/// Stacks two expression, with the first possibly missing. If the two
/// expressions are present, then an application node is created.
fn stack_exprs(left: Option<Expr>, right: Expr) -> Expr {
    match left {
        Some(function) => Expr {
            span: function.span.join(&right.span),
            kind: ExprKind::Application {
                function: Box::new(function),
                argument: Box::new(right),
            },
        },

        None => right,
    }
}
