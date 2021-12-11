/// Exports items related to the lambda calculus lexer.

pub mod error;

use crate::token::TokenKind;
use bittongue::{
    diagnostic::Diagnostics,
    lexer::{Lexer as LexerTrait, LexingError, Token},
    source::Reader,
};
use error::InvalidGrapheme;

/// Lexer implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lexer;

impl LexerTrait for Lexer {
    type TokenKind = TokenKind;

    fn generate_token(
        &mut self,
        reader: &mut Reader,
        diagnostics: &mut Diagnostics,
    ) -> Result<Token<Self::TokenKind>, LexingError> {
        self.skip_discardable(reader);

        reader.mark();

        if self.is_curr_ident(reader) {
            Ok(self.tokenize_ident(reader))
        } else if self.is_curr_lambda(reader) {
            Ok(self.tokenize_lambda(reader))
        } else if self.is_curr_dot(reader) {
            Ok(self.tokenize_dot(reader))
        } else if self.is_curr_open_paren(reader) {
            Ok(self.tokenize_open_paren(reader))
        } else if self.is_curr_close_paren(reader) {
            Ok(self.tokenize_close_paren(reader))
        } else if reader.is_eof() {
            Ok(self.tokenize_eof(reader))
        } else {
            self.invalid_grapheme(reader, diagnostics);
            Err(LexingError)
        }
    }
}

impl Lexer {
    /// Is the reader's current grapheme an identifier letter?
    ///
    /// Digits allowed even at the beginning of the identifier.
    fn is_curr_ident(&self, reader: &mut Reader) -> bool {
        reader.test(|grapheme| {
            grapheme.is_ascii_alphanumeric() || grapheme == "_"
        })
    }

    /// Is the reader's current grapheme a lambda sign?
    fn is_curr_lambda(&self, reader: &mut Reader) -> bool {
        reader.test(|grapheme| grapheme == "\\")
    }

    /// Is the reader's current grapheme a dot?
    fn is_curr_dot(&self, reader: &mut Reader) -> bool {
        reader.test(|grapheme| grapheme == ".")
    }

    /// Is the reader's current grapheme an opening parenthesis?
    fn is_curr_open_paren(&self, reader: &mut Reader) -> bool {
        reader.test(|grapheme| grapheme == "(")
    }

    /// Is the reader's current grapheme a closing parenthesis?
    fn is_curr_close_paren(&self, reader: &mut Reader) -> bool {
        reader.test(|grapheme| grapheme == ")")
    }

    /// Is the reader's current grapheme whitespace?
    fn is_curr_whitespace(&self, reader: &Reader) -> bool {
        reader.test(|grapheme| grapheme.is_whitespace_char())
    }

    /// Is the reader's current grapheme a comment start (`;`)?
    fn is_curr_comment_start(&self, reader: &Reader) -> bool {
        reader.test(|grapheme| grapheme == ";")
    }

    /// Is the reader's current grapheme a comment ending (`\n` or EOF)?
    fn is_curr_comment_end(&self, reader: &Reader) -> bool {
        reader.test_or_eof(|grapheme| grapheme == "\n")
    }

    /// Discards unused grapheme sequences, such as whitespaces and comments.
    fn skip_discardable(&self, reader: &mut Reader) {
        while self.skip_whitespace(reader) || self.skip_comment(reader) {}
    }

    /// Discards whitespaces. Returns whether any whitespace was found.
    fn skip_whitespace(&self, reader: &mut Reader) -> bool {
        let mut skipped = false;
        while self.is_curr_whitespace(reader) {
            reader.next();
            skipped = true;
        }
        skipped
    }

    /// Discards comments. Returns whether a comment was found.
    fn skip_comment(&self, reader: &mut Reader) -> bool {
        if self.is_curr_comment_start(reader) {
            while !self.is_curr_comment_end(reader) {
                reader.next();
            }
            true
        } else {
            false
        }
    }

    /// Tokenizes an identifier.
    fn tokenize_ident(&self, reader: &mut Reader) -> Token<TokenKind> {
        while self.is_curr_ident(reader) {
            reader.next();
        }
        Token { kind: TokenKind::Ident, span: reader.span() }
    }

    /// Tokenizes a lambda sign.
    fn tokenize_lambda(&self, reader: &mut Reader) -> Token<TokenKind> {
        reader.next();
        Token { kind: TokenKind::Lambda, span: reader.span() }
    }

    /// Tokenizes a dot.
    fn tokenize_dot(&self, reader: &mut Reader) -> Token<TokenKind> {
        reader.next();
        Token { kind: TokenKind::Dot, span: reader.span() }
    }

    /// Tokenizes an opening parenthesis.
    fn tokenize_open_paren(&self, reader: &mut Reader) -> Token<TokenKind> {
        reader.next();
        Token { kind: TokenKind::OpenParen, span: reader.span() }
    }

    /// Tokenizes a closing parenthesis.
    fn tokenize_close_paren(&self, reader: &mut Reader) -> Token<TokenKind> {
        reader.next();
        Token { kind: TokenKind::CloseParen, span: reader.span() }
    }

    /// Tokenizes an end-of-input mark.
    fn tokenize_eof(&self, reader: &mut Reader) -> Token<TokenKind> {
        Token { kind: TokenKind::Eof, span: reader.span() }
    }

    /// Raises an error for an invalid grapheme.
    fn invalid_grapheme(
        &self,
        reader: &mut Reader,
        diagnostics: &mut Diagnostics,
    ) {
        reader.next();
        diagnostics.raise(InvalidGrapheme { span: reader.span() });
    }
}
