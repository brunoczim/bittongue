use crate::{
    ast::{Expr, ExprKind},
    error::{MismatchedToken, UnmatchedCloseParen, UnmatchedOpenParen},
    lexer::Lexer,
    token::TokenKind,
};
use bittongue::{
    diagnostic::Diagnostics,
    lexer::{Token, TokenStream},
};

#[derive(Debug, Clone, Copy, Default)]
pub struct ParseError;

pub fn parse_expr(
    token_stream: &mut TokenStream<Lexer>,
    diagnostics: &mut Diagnostics,
) -> Result<Expr, ParseError> {
    parse_expr_with_end(TokenKind::Eof, token_stream, diagnostics)
}

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

fn try_expect_token(
    expected: &[TokenKind],
    token_stream: &mut TokenStream<Lexer>,
    diagnostics: &mut Diagnostics,
) -> Option<Token<TokenKind>> {
    let current = token_stream.current().map(Clone::clone);
    match current {
        Ok(token) => {
            if expected.contains(&token.kind) {
                token_stream.next(diagnostics);
                Some(token)
            } else {
                None
            }
        },
        Err(_) => {
            token_stream.next(diagnostics);
            None
        },
    }
}

fn parse_expr_with_end(
    end: TokenKind,
    token_stream: &mut TokenStream<Lexer>,
    diagnostics: &mut Diagnostics,
) -> Result<Expr, ParseError> {
    let mut curr_expr = None;
    let mut had_child_token = false;

    loop {
        match token_stream.current() {
            Ok(token) => match token.kind {
                kind if end == kind => break,
                TokenKind::Lambda => {
                    if let Ok(lambda) =
                        parse_lambda(end, token_stream, diagnostics)
                    {
                        curr_expr = Some(stack_exprs(curr_expr, lambda));
                    }
                },
                TokenKind::Ident => {
                    let next_expr = Expr {
                        span: token.span.clone(),
                        kind: ExprKind::Variable(token.span.clone()),
                    };
                    curr_expr = Some(stack_exprs(curr_expr, next_expr));
                    token_stream.next(diagnostics);
                },

                TokenKind::OpenParen => {
                    let open_paren = token.clone();
                    token_stream.next(diagnostics);
                    if let Ok(mut next_expr) = parse_expr_with_end(
                        TokenKind::CloseParen,
                        token_stream,
                        diagnostics,
                    ) {
                        let close_paren_res = try_expect_token(
                            &[TokenKind::CloseParen],
                            token_stream,
                            diagnostics,
                        );
                        next_expr.span = next_expr.span.join(&open_paren.span);
                        if let Some(close_paren) = close_paren_res {
                            next_expr.span =
                                next_expr.span.join(&close_paren.span);
                        } else {
                            let error =
                                UnmatchedOpenParen { span: open_paren.span };
                            diagnostics.raise(error);
                        }
                        curr_expr = Some(stack_exprs(curr_expr, next_expr));
                    }
                },

                TokenKind::Eof => break,

                TokenKind::CloseParen => {
                    let error =
                        UnmatchedCloseParen { span: token.span.clone() };
                    diagnostics.raise(error);
                    token_stream.next(diagnostics);
                },

                _ => {
                    let error = MismatchedToken {
                        expected: vec![
                            TokenKind::Lambda,
                            TokenKind::Ident,
                            TokenKind::OpenParen,
                            end,
                        ],
                        found: token.clone(),
                    };
                    diagnostics.raise(error);
                    token_stream.next(diagnostics);
                },
            },

            Err(_) => {
                token_stream.next(diagnostics);
            },
        }

        had_child_token = true;
    }

    match curr_expr {
        Some(expr) => Ok(expr),
        None => {
            if !had_child_token {
                let error = MismatchedToken {
                    expected: vec![
                        TokenKind::Lambda,
                        TokenKind::Ident,
                        TokenKind::OpenParen,
                    ],
                    found: token_stream.current().unwrap().clone(),
                };
                diagnostics.raise(error);
            }
            Err(ParseError)
        },
    }
}

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
    let parameter_result =
        expect_token(&[TokenKind::Ident], token_stream, diagnostics);
    let _ = expect_token(&[TokenKind::Dot], token_stream, diagnostics);
    let body = parse_expr_with_end(end, token_stream, diagnostics)?;
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
