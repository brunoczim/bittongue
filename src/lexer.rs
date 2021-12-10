use crate::{
    diagnostic::Diagnostics,
    source::{Reader, Source, Span},
};

pub trait TokenKind {
    fn is_eof(&self) -> bool;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LexingError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token<K>
where
    K: TokenKind,
{
    pub kind: K,
    pub span: Span,
}

pub trait Lexer {
    type TokenKind: TokenKind;

    fn generate_token(
        &mut self,
        reader: &mut Reader,
        diagnostics: &mut Diagnostics,
    ) -> Result<Token<Self::TokenKind>, LexingError>;
}

impl<'this, L> Lexer for &'this mut L
where
    L: Lexer + ?Sized,
{
    type TokenKind = L::TokenKind;

    fn generate_token(
        &mut self,
        reader: &mut Reader,
        diagnostics: &mut Diagnostics,
    ) -> Result<Token<Self::TokenKind>, LexingError> {
        (**self).generate_token(reader, diagnostics)
    }
}

impl<L> Lexer for Box<L>
where
    L: Lexer + ?Sized,
{
    type TokenKind = L::TokenKind;

    fn generate_token(
        &mut self,
        reader: &mut Reader,
        diagnostics: &mut Diagnostics,
    ) -> Result<Token<Self::TokenKind>, LexingError> {
        (**self).generate_token(reader, diagnostics)
    }
}

#[derive(Debug, Clone)]
pub struct TokenStream<L>
where
    L: Lexer,
{
    reader: Reader,
    lexer: L,
    tokens: Vec<Result<Token<L::TokenKind>, LexingError>>,
    position: usize,
}

impl<L> TokenStream<L>
where
    L: Lexer,
{
    pub fn new(
        source: &Source,
        lexer: L,
        diagnostics: &mut Diagnostics,
    ) -> Self {
        let mut this = Self {
            reader: source.reader(),
            lexer,
            tokens: Vec::new(),
            position: 0,
        };
        let tok_res = this.lexer.generate_token(&mut this.reader, diagnostics);
        this.tokens.push(tok_res);
        this
    }

    pub fn is_eof(&self) -> bool {
        self.current().map_or(false, |token| token.kind.is_eof())
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn current(&self) -> Result<&Token<L::TokenKind>, LexingError> {
        self.tokens[self.position].as_ref().map_err(|error| *error)
    }

    pub fn source(&self) -> &Source {
        self.reader.source()
    }

    pub fn next(&mut self, diagnostics: &mut Diagnostics) -> bool {
        if self.is_eof() {
            false
        } else {
            self.position += 1;
            if self.position >= self.tokens.len() {
                let tok_res =
                    self.lexer.generate_token(&mut self.reader, diagnostics);
                self.tokens.push(tok_res);
            }
            true
        }
    }

    pub fn prev(&mut self) -> bool {
        if self.position == 0 {
            false
        } else {
            self.position -= 1;
            true
        }
    }

    pub fn advance(
        &mut self,
        count: usize,
        diagnostics: &mut Diagnostics,
    ) -> usize {
        let mut advanced = count.min(self.tokens.len() - self.position);
        self.position += advanced;
        while advanced < count && self.next(diagnostics) {
            advanced += 1;
        }
        advanced
    }

    pub fn rollback(&mut self, count: usize) -> usize {
        let rolled = count.min(self.position);
        self.position -= rolled;
        rolled
    }
}
