//! This module defines token items necessary for lambda calculus parsing.

use bittongue::lexer::TokenKind as TokenKindTrait;
use std::fmt;

/// The kind of tokens used by lambda calculus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenKind {
    /// `abc_def_0123`
    Ident,
    /// `\`
    Lambda,
    /// `.`
    Dot,
    /// `(`
    OpenParen,
    /// `)`
    CloseParen,
    /// `` (end of file/input)
    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.write_str(match self {
            TokenKind::Ident => "<identifier>",
            TokenKind::Lambda => "`\\`",
            TokenKind::Dot => "`.`",
            TokenKind::OpenParen => "`(`",
            TokenKind::CloseParen => "`)`",
            TokenKind::Eof => "end of input",
        })
    }
}

impl TokenKindTrait for TokenKind {
    fn is_eof(&self) -> bool {
        *self == TokenKind::Eof
    }
}
