//! Errors raised by the parser.

use crate::token::TokenKind;
use bittongue::{
    diagnostic::{Diagnostic, Level},
    lexer::Token,
    source::Span,
};
use std::fmt;

/// Error raised when the parser expected a set of tokens, but an unexpected
/// token was found.
#[derive(Debug, Clone)]
pub struct MismatchedToken {
    /// Expected set of tokens.
    pub expected: Vec<TokenKind>,
    /// Unexpected token that was found.
    pub found: Token<TokenKind>,
}

impl fmt::Display for MismatchedToken {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "expected ")?;

        match self.expected.split_first() {
            Some((first, tail)) => {
                write!(fmtr, "{}", first)?;
                if let Some((last, init)) = tail.split_last() {
                    for kind in init {
                        write!(fmtr, ", {}", kind)?;
                    }
                    write!(fmtr, " or {}", last)?;
                }
            },

            None => write!(fmtr, "nothing")?,
        }

        write!(fmtr, ", found {}", self.found)?;

        Ok(())
    }
}

impl Diagnostic for MismatchedToken {
    fn level(&self) -> Level {
        // Make this always a hard error.
        Level::Error
    }

    // If anyone want to print line and column number, use the Span yielded by
    // this method.
    fn primary_span(&self) -> Option<Span> {
        Some(self.found.span.clone())
    }
}

/// Error raised when the parser expected a closing parenthesis matching an
/// existing open parenthesis, but no such closing parenthesis was found.
#[derive(Debug, Clone)]
pub struct UnmatchedOpenParen {
    /// Span of the opening parenthesis.
    pub span: Span,
}

impl fmt::Display for UnmatchedOpenParen {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "unmatched opening parenthesis `{}`", self.span.as_str())
    }
}

impl Diagnostic for UnmatchedOpenParen {
    fn level(&self) -> Level {
        Level::Error
    }

    fn primary_span(&self) -> Option<Span> {
        Some(self.span.clone())
    }
}

/// Error raised when the parser expected an opening parenthesis matching an
/// existing closing parenthesis, but no such opening parenthesis was found.
#[derive(Debug, Clone)]
pub struct UnmatchedCloseParen {
    /// Span of the closing parenthesis.
    pub span: Span,
}

impl fmt::Display for UnmatchedCloseParen {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "unmatched closing parenthesis `{}`", self.span.as_str())
    }
}

impl Diagnostic for UnmatchedCloseParen {
    fn level(&self) -> Level {
        Level::Error
    }

    fn primary_span(&self) -> Option<Span> {
        Some(self.span.clone())
    }
}
