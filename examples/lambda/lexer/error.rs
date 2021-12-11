//! Errors raised by the lexer.

use bittongue::{
    diagnostic::{Diagnostic, Level},
    source::Span,
};
use std::fmt;

/// Error raised when the lexer finds an invalid grapheme cluster.
#[derive(Debug, Clone)]
pub struct InvalidGrapheme {
    /// Span of such cluster.
    pub span: Span,
}

impl fmt::Display for InvalidGrapheme {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        // Do not concern about displaying line and column numbers here.
        write!(fmtr, "invalid grapheme cluster {:#?}", self.span.as_str())
    }
}

impl Diagnostic for InvalidGrapheme {
    fn level(&self) -> Level {
        // Make this always a hard error.
        Level::Error
    }

    // If anyone want to print line and column number, use the Span yielded by
    // this method.
    fn primary_span(&self) -> Option<Span> {
        Some(self.span.clone())
    }
}
