use bittongue::lexer::TokenKind as TokenKindTrait;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenKind {
    Ident,
    Lambda,
    Dot,
    OpenParen,
    CloseParen,
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
