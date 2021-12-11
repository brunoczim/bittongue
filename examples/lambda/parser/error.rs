use crate::token::TokenKind;
use bittongue::{
    diagnostic::{Diagnostic, Level},
    lexer::Token,
    source::Span,
};
use std::fmt;

/// Error raised when the lexer finds an invalid grapheme cluster.
#[derive(Debug, Clone)]
pub struct MismatchedToken {
    pub expected: Vec<TokenKind>,
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

#[derive(Debug, Clone)]
pub struct UnmatchedOpenParen {
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

#[derive(Debug, Clone)]
pub struct UnmatchedCloseParen {
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
