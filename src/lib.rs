//! This crate provides helpers for source code management when writing
//! programming languages (or non-programming computer languages).
//!
//! # Examples
//!
//! ## Lexer
//!
//! ```
//! use std::fmt;
//! use bittongue::diagnostic::{Diagnostic, Diagnostics, Level};
//! use bittongue::source::{Source, Reader, Span};
//! use bittongue::lexer::{
//!     Token,
//!     TokenStream,
//!     LexingError,
//!     TokenKind as TokenKindTrait,
//!     Lexer as LexerTrait,
//! };
//!
//! fn main() {
//!     let source = Source::new("example.lisp", "; addition\n(add ç (x y))");
//!     let mut diagnostics = Diagnostics::new();
//!     let mut tokens = TokenStream::new(&source, Lexer, &mut diagnostics);
//!     assert!(diagnostics.is_ok());
//!
//!     let token = tokens.current().unwrap();
//!     assert_eq!(token.kind, TokenKind::OpenParen);
//!     assert_eq!(token.span.as_str(), "(");
//!     // Starts from zero
//!     assert_eq!(token.span.start().line(), 1);
//!     assert_eq!(token.span.start().column(), 0);
//!     assert_eq!(token.span.end().line(), 1);
//!     assert_eq!(token.span.end().column(), 1);
//!
//!     assert!(tokens.next(&mut diagnostics));
//!     assert!(diagnostics.is_ok());
//!
//!     let token = tokens.current().unwrap();
//!     assert_eq!(token.kind, TokenKind::Identifier);
//!     assert_eq!(token.span.as_str(), "add");
//!     // Starts from zero
//!     assert_eq!(token.span.start().line(), 1);
//!     assert_eq!(token.span.start().column(), 1);
//!     assert_eq!(token.span.end().line(), 1);
//!     assert_eq!(token.span.end().column(), 4);
//!
//!     assert!(tokens.next(&mut diagnostics));
//!     assert!(diagnostics.is_err());
//!     tokens.current().unwrap_err();
//!
//!     assert!(tokens.next(&mut diagnostics));
//!     assert!(diagnostics.is_err());
//!
//!     let token = tokens.current().unwrap();
//!     assert_eq!(token.kind, TokenKind::OpenParen);
//!     assert_eq!(token.span.as_str(), "(");
//!     // Starts from zero
//!     assert_eq!(token.span.start().line(), 1);
//!     assert_eq!(token.span.start().column(), 7);
//!     assert_eq!(token.span.end().line(), 1);
//!     assert_eq!(token.span.end().column(), 8);
//!
//!     assert!(tokens.next(&mut diagnostics));
//!     assert!(diagnostics.is_err());
//!
//!     let token = tokens.current().unwrap();
//!     assert_eq!(token.kind, TokenKind::Identifier);
//!     assert_eq!(token.span.as_str(), "x");
//!     // Starts from zero
//!     assert_eq!(token.span.start().line(), 1);
//!     assert_eq!(token.span.start().column(), 8);
//!     assert_eq!(token.span.end().line(), 1);
//!     assert_eq!(token.span.end().column(), 9);
//!
//!     assert!(tokens.next(&mut diagnostics));
//!     assert!(diagnostics.is_err());
//!
//!     let token = tokens.current().unwrap();
//!     assert_eq!(token.kind, TokenKind::Identifier);
//!     assert_eq!(token.span.as_str(), "y");
//!     // Starts from zero
//!     assert_eq!(token.span.start().line(), 1);
//!     assert_eq!(token.span.start().column(), 10);
//!     assert_eq!(token.span.end().line(), 1);
//!     assert_eq!(token.span.end().column(), 11);
//!
//!     assert!(tokens.next(&mut diagnostics));
//!     assert!(diagnostics.is_err());
//!
//!     let token = tokens.current().unwrap();
//!     assert_eq!(token.kind, TokenKind::CloseParen);
//!     assert_eq!(token.span.as_str(), ")");
//!     // Starts from zero
//!     assert_eq!(token.span.start().line(), 1);
//!     assert_eq!(token.span.start().column(), 11);
//!     assert_eq!(token.span.end().line(), 1);
//!     assert_eq!(token.span.end().column(), 12);
//!
//!     assert!(tokens.next(&mut diagnostics));
//!     assert!(diagnostics.is_err());
//!
//!     let token = tokens.current().unwrap();
//!     assert_eq!(token.kind, TokenKind::CloseParen);
//!     assert_eq!(token.span.as_str(), ")");
//!     // Starts from zero
//!     assert_eq!(token.span.start().line(), 1);
//!     assert_eq!(token.span.start().column(), 12);
//!     assert_eq!(token.span.end().line(), 1);
//!     assert_eq!(token.span.end().column(), 13);
//!
//!     assert!(tokens.next(&mut diagnostics));
//!     assert!(diagnostics.is_err());
//!
//!     let token = tokens.current().unwrap();
//!     assert_eq!(token.kind, TokenKind::Eof);
//!     assert_eq!(token.span.as_str(), "");
//!     // Starts from zero
//!     assert_eq!(token.span.start().line(), 1);
//!     assert_eq!(token.span.start().column(), 13);
//!     assert_eq!(token.span.end().line(), 1);
//!     assert_eq!(token.span.end().column(), 13);
//!
//!     assert!(!tokens.next(&mut diagnostics));
//! }
//!
//! #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
//! pub enum TokenKind {
//!     Identifier,
//!     OpenParen,
//!     CloseParen,
//!     Eof,
//! }
//!
//! impl TokenKindTrait for TokenKind {
//!     fn is_eof(&self) -> bool {
//!         *self == TokenKind::Eof
//!     }
//! }
//!
//! #[derive(Debug, Clone)]
//! pub struct InvalidGrapheme {
//!     pub span: Span,
//! }
//!
//! impl fmt::Display for InvalidGrapheme {
//!     fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
//!         write!(fmtr, "invalid grapheme cluster {:#?}", self.span.as_str())
//!     }
//! }
//!
//! impl Diagnostic for InvalidGrapheme {
//!     fn level(&self) -> Level {
//!         Level::Error
//!     }
//!
//!     fn primary_span(&self) -> Option<Span> {
//!         Some(self.span.clone())
//!     }
//! }
//!
//! #[derive(Debug, Clone)]
//! pub struct Lexer;
//!
//! impl Lexer {
//!     fn is_curr_ident(&self, reader: &Reader) -> bool {
//!         reader.current().map_or(false, |grapheme| {
//!             grapheme.len() == 1 && (
//!                 grapheme >= "A" && grapheme <= "Z"
//!                 || grapheme >= "a" && grapheme <= "z"
//!                 || grapheme >= "0" && grapheme <= "9"
//!             )
//!         })
//!     }
//!
//!     fn is_curr_open_paren(&self, reader: &Reader) -> bool {
//!         reader.current().map_or(false, |grapheme| grapheme == "(")
//!     }
//!
//!     fn is_curr_close_paren(&self, reader: &Reader) -> bool {
//!         reader.current().map_or(false, |grapheme| grapheme == ")")
//!     }
//!
//!     fn is_curr_whitespace(&self, reader: &Reader) -> bool {
//!         reader.current().map_or(false, |grapheme|
//!             grapheme.chars().all(char::is_whitespace)
//!         )
//!     }
//!
//!     fn is_curr_comment_start(&self, reader: &Reader) -> bool {
//!         reader.current().map_or(false, |grapheme| grapheme == ";")
//!     }
//!
//!     fn is_curr_comment_end(&self, reader: &Reader) -> bool {
//!         reader.current().map_or(true, |grapheme| grapheme == "\n")
//!     }
//!
//!     fn skip_discardable(&self, reader: &mut Reader) {
//!         while self.skip_whitespace(reader) || self.skip_comment(reader) {}
//!     }
//!
//!     fn skip_whitespace(&self, reader: &mut Reader) -> bool {
//!         let mut skipped = false;
//!         while self.is_curr_whitespace(reader) {
//!             reader.next();
//!             skipped = true;
//!         }
//!         skipped
//!     }
//!
//!     fn skip_comment(&self, reader: &mut Reader) -> bool {
//!         if self.is_curr_comment_start(reader) {
//!             while !self.is_curr_comment_end(reader) {
//!                 reader.next();
//!             }
//!             true
//!         } else {
//!             false
//!         }
//!     }
//! }
//!
//! impl LexerTrait for Lexer {
//!     type TokenKind = TokenKind;
//!
//!     fn generate_token(
//!         &mut self,
//!         reader: &mut Reader,
//!          diagnostics: &mut Diagnostics,
//!     ) -> Result<Token<Self::TokenKind>, LexingError> {
//!         self.skip_discardable(reader);
//!
//!         reader.mark();
//!         if self.is_curr_ident(reader) {
//!             while self.is_curr_ident(reader) {
//!                 reader.next();
//!             }
//!             Ok(Token {
//!                 kind: TokenKind::Identifier,
//!                 span: reader.span(),
//!             })
//!         } else if self.is_curr_open_paren(reader) {
//!             reader.next();
//!             Ok(Token {
//!                 kind: TokenKind::OpenParen,
//!                 span: reader.span(),
//!             })
//!         } else if self.is_curr_close_paren(reader) {
//!             reader.next();
//!             Ok(Token {
//!                 kind: TokenKind::CloseParen,
//!                 span: reader.span(),
//!             })
//!         } else if reader.is_eof() {
//!             Ok(Token {
//!                 kind: TokenKind::Eof,
//!                 span: reader.span(),
//!             })
//!         } else {
//!             reader.next();
//!             diagnostics.raise(InvalidGrapheme { span: reader.span() });
//!             Err(LexingError)
//!         }
//!     }
//! }
//! ```

pub mod diagnostic;
pub mod source;
pub mod lexer;
