//! This crate provides helpers for source code management when writing
//! programming languages (or non-programming computer languages).
//!
//! # Examples
//!
//! ## Lexer
//!
//! This example is just a lexer of a simple language. The language only has
//! three types of tokens: identifiers, opening parenthesis, closing
//! parenthesis.
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
//!     // Creates the source code object, segmented in terms of grapheme clusters.
//!     let source = Source::new("example.lang", "; addition\n(add ç (x y))");
//!     // Parser/compiler diagnostics collection.
//!     let mut diagnostics = Diagnostics::new();
//!     // Token stream that generates tokens using our custom lexer.
//!     let mut tokens = TokenStream::new(&source, Lexer, &mut diagnostics);
//!     // First token must have been already generated, and it was success.
//!     assert!(diagnostics.is_ok());
//!
//!     // First token
//!     let token = tokens.current().unwrap();
//!     assert_eq!(token.kind, TokenKind::OpenParen);
//!     assert_eq!(token.span.as_str(), "(");
//!     // Starts from zero
//!     assert_eq!(token.span.start().line(), 1);
//!     assert_eq!(token.span.start().column(), 0);
//!     assert_eq!(token.span.end().line(), 1);
//!     assert_eq!(token.span.end().column(), 1);
//!
//!     // Generate next token, must be a success
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
//!     // We now reach an invalid character, we implemented the lexer such that this
//!     // is an error.
//!     assert!(tokens.next(&mut diagnostics));
//!     // And so our diagnostics collection has an error now.
//!     assert!(diagnostics.is_err());
//!     assert!(tokens.current().is_err());
//!
//!     // Next token will be a success, but the diagnostics collection will still
//!     // contain an error.
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
//!     // Again, next token is successful, but the error will persist.
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
//!     // Success again (even though the error persists).
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
//!     // Success again (even though the error persists).
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
//!     // Finally, the last token before End-Of-File.
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
//!     // This will generate the End-Of-File.
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
//!     // As EOF is reached, no more tokens will be generated.
//!     assert!(!tokens.next(&mut diagnostics));
//! }
//!
//! /// The kind of tokens present in our programming language.
//! #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
//! pub enum TokenKind {
//!     /// An identifier (i.e. alphanumeric characters or _). We will allow
//!     /// identifiers to begin with a digit though.
//!     Identifier,
//!     /// Just '('
//!     OpenParen,
//!     /// Just ')'
//!     CloseParen,
//!     /// Marks end of input.
//!     Eof,
//! }
//!
//! // Required for the lexer.
//! impl TokenKindTrait for TokenKind {
//!     fn is_eof(&self) -> bool {
//!         *self == TokenKind::Eof
//!     }
//! }
//!
//! /// Error raised when the lexer finds an invalid grapheme cluster.
//! #[derive(Debug, Clone)]
//! pub struct InvalidGrapheme {
//!     /// Span of such cluster.
//!     pub span: Span,
//! }
//!
//! impl fmt::Display for InvalidGrapheme {
//!     fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
//!         // Do not concern about displaying line and column numbers here.
//!         write!(fmtr, "invalid grapheme cluster {:#?}", self.span.as_str())
//!     }
//! }
//!
//! impl Diagnostic for InvalidGrapheme {
//!     fn level(&self) -> Level {
//!         // Make this always a hard error.
//!         Level::Error
//!     }
//!
//!     // If anyone want to print line and column number, use the Span yielded by
//!     // this method.
//!     fn primary_span(&self) -> Option<Span> {
//!         Some(self.span.clone())
//!     }
//! }
//!
//! #[derive(Debug, Clone)]
//! pub struct Lexer;
//!
//! impl LexerTrait for Lexer {
//!     type TokenKind = TokenKind;
//!
//!     fn generate_token(
//!         &mut self,
//!         reader: &mut Reader,
//!         diagnostics: &mut Diagnostics,
//!     ) -> Result<Token<Self::TokenKind>, LexingError> {
//!         // Skips all whitespace and comments
//!         self.skip_discardable(reader);
//!
//!         // Marks the beginning of the token.
//!         reader.mark();
//!         
//!         // Dispatch tokenization for each kind of token.
//!         if self.is_curr_ident(reader) {
//!             Ok(self.tokenize_ident(reader))
//!         } else if self.is_curr_open_paren(reader) {
//!             Ok(self.tokenize_open_paren(reader))
//!         } else if self.is_curr_close_paren(reader) {
//!             Ok(self.tokenize_close_paren(reader))
//!         } else if reader.is_eof() {
//!             Ok(self.tokenize_eof(reader))
//!         } else {
//!             // If no token kind is matched, then this character is invalid.
//!             self.invalid_grapheme(reader, diagnostics);
//!             Err(LexingError)
//!         }
//!     }
//! }
//!
//! impl Lexer {
//!     /// Tests for identifier graphemes.
//!     fn is_curr_ident(&self, reader: &Reader) -> bool {
//!         reader.test(|graph| graph.is_ascii_alphanumeric() || graph == "_")
//!     }
//!
//!     /// Tests for opening parenthesis.
//!     fn is_curr_open_paren(&self, reader: &Reader) -> bool {
//!         reader.test(|grapheme| grapheme == "(")
//!     }
//!
//!     /// Tests for closing parenthesis.
//!     fn is_curr_close_paren(&self, reader: &Reader) -> bool {
//!         reader.test(|grapheme| grapheme == ")")
//!     }
//!
//!     /// Tests for whitespaces.
//!     fn is_curr_whitespace(&self, reader: &Reader) -> bool {
//!         reader.test(|grapheme| grapheme.is_whitespace_char())
//!     }
//!
//!     /// Tests for comment starts (`;`).
//!     fn is_curr_comment_start(&self, reader: &Reader) -> bool {
//!         reader.test(|grapheme| grapheme == ";")
//!     }
//!
//!     /// Tests for comment ends (`\n` or end-of-input).
//!     fn is_curr_comment_end(&self, reader: &Reader) -> bool {
//!         reader.test_or_eof(|grapheme| grapheme == "\n")
//!     }
//!
//!     /// Discards unused grapheme sequences, such as whitespaces and comments.
//!     fn skip_discardable(&self, reader: &mut Reader) {
//!         while self.skip_whitespace(reader) || self.skip_comment(reader) {}
//!     }
//!
//!     /// Discards whitespaces. Returns whether any whitespace was found.
//!     fn skip_whitespace(&self, reader: &mut Reader) -> bool {
//!         let mut skipped = false;
//!         while self.is_curr_whitespace(reader) {
//!             reader.next();
//!             skipped = true;
//!         }
//!         skipped
//!     }
//!
//!     /// Discards comments. Returns whether a comment was found.
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
//!
//!     /// Produces an identifier token.
//!     fn tokenize_ident(&self, reader: &mut Reader) -> Token<TokenKind> {
//!         while self.is_curr_ident(reader) {
//!             reader.next();
//!         }
//!         Token { kind: TokenKind::Identifier, span: reader.span() }
//!     }
//!
//!     /// Produces an opening parenthesis token.
//!     fn tokenize_open_paren(&self, reader: &mut Reader) -> Token<TokenKind> {
//!         reader.next();
//!         Token { kind: TokenKind::OpenParen, span: reader.span() }
//!     }
//!
//!     /// Produces a closing parenthesis token.
//!     fn tokenize_close_paren(&self, reader: &mut Reader) -> Token<TokenKind> {
//!         reader.next();
//!         Token { kind: TokenKind::CloseParen, span: reader.span() }
//!     }
//!
//!     /// Produces an End-Of-File token.
//!     fn tokenize_eof(&self, reader: &mut Reader) -> Token<TokenKind> {
//!         Token { kind: TokenKind::Eof, span: reader.span() }
//!     }
//!
//!     /// Raises an error for an invalid grapheme.
//!     fn invalid_grapheme(
//!         &self,
//!         reader: &mut Reader,
//!         diagnostics: &mut Diagnostics,
//!     ) {
//!         reader.next();
//!         diagnostics.raise(InvalidGrapheme { span: reader.span() });
//!     }
//! }
//! ```

pub mod grapheme;
pub mod diagnostic;
pub mod source;
pub mod lexer;
