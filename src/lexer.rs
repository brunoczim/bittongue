use crate::{
    diagnostic::{Diagnostic, Diagnostics},
    source::Reader,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct LexingError;

pub trait Lexer {
    type Token;

    fn transit(
        &mut self,
        current: Option<&str>,
        diagnostics: &mut Diagnostics,
    ) -> Option<Result<Self::Token, LexingError>>;
}

#[derive(Debug, Clone)]
pub struct TokenStream<L>
where
    L: Lexer,
{
    reader: Reader,
    lexer: L,
    tokens: Vec<Result<L::Token, LexingError>>,
    curr_token: usize,
}

impl<L> TokenStream<L>
where
    L: Lexer,
{
    pub fn new(reader: Reader, lexer: L) -> Self {
        let mut this =
            Self { reader, lexer, tokens: Vec::new(), curr_token: 0 };
        this.gen_curr_token();
        this
    }

    pub fn current(&self) -> Result<&L::Token, LexingError> {
        self.tokens[self.curr_token].as_ref().map_err(|err| *err)
    }

    fn gen_curr_token(&mut self) {
        todo!()
    }
}
