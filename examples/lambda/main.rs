use bittongue::{
    diagnostic::{Diagnostic, Diagnostics, Level},
    lexer::{
        Lexer as LexerTrait,
        LexingError,
        Token,
        TokenKind as TokenKindTrait,
        TokenStream,
    },
    source::{Reader, Source, Span},
};
use std::{env, fmt, fs, io, path::Path, process};

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
    fn is_curr_ident(&self, reader: &mut Reader) -> bool {
        reader.test(|grapheme| {
            grapheme.is_ascii_alphanumeric() || grapheme == "_"
        })
    }

    fn is_curr_lambda(&self, reader: &mut Reader) -> bool {
        reader.test(|grapheme| grapheme == "\\")
    }

    fn is_curr_dot(&self, reader: &mut Reader) -> bool {
        reader.test(|grapheme| grapheme == ".")
    }

    fn is_curr_open_paren(&self, reader: &mut Reader) -> bool {
        reader.test(|grapheme| grapheme == "(")
    }

    fn is_curr_close_paren(&self, reader: &mut Reader) -> bool {
        reader.test(|grapheme| grapheme == ")")
    }

    /// Tests for whitespaces.
    fn is_curr_whitespace(&self, reader: &Reader) -> bool {
        reader.test(|grapheme| grapheme.is_whitespace_char())
    }

    /// Tests for comment starts (`;`).
    fn is_curr_comment_start(&self, reader: &Reader) -> bool {
        reader.test(|grapheme| grapheme == ";")
    }

    /// Tests for comment ends (`\n` or end-of-input).
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

    fn tokenize_ident(&self, reader: &mut Reader) -> Token<TokenKind> {
        while self.is_curr_ident(reader) {
            reader.next();
        }
        Token { kind: TokenKind::Ident, span: reader.span() }
    }

    fn tokenize_lambda(&self, reader: &mut Reader) -> Token<TokenKind> {
        reader.next();
        Token { kind: TokenKind::Lambda, span: reader.span() }
    }

    fn tokenize_dot(&self, reader: &mut Reader) -> Token<TokenKind> {
        reader.next();
        Token { kind: TokenKind::Dot, span: reader.span() }
    }

    fn tokenize_open_paren(&self, reader: &mut Reader) -> Token<TokenKind> {
        reader.next();
        Token { kind: TokenKind::OpenParen, span: reader.span() }
    }

    fn tokenize_close_paren(&self, reader: &mut Reader) -> Token<TokenKind> {
        reader.next();
        Token { kind: TokenKind::CloseParen, span: reader.span() }
    }

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Expr {
    pub span: Span,
    pub kind: ExprKind,
}

impl fmt::Display for Expr {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}", DisplayExpr { expr: self, level: 0 })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExprKind {
    Variable(Span),
    Application { function: Box<Expr>, argument: Box<Expr> },
    Lambda { parameter: Span, body: Box<Expr> },
}

impl fmt::Display for ExprKind {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}", DisplayExprKind { kind: self, level: 0 })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ParseError;

pub fn parse_expr(
    token_stream: &mut TokenStream<Lexer>,
    diagnostics: &mut Diagnostics,
) -> Result<Expr, ParseError> {
    parse_expr_with_end(TokenKind::Eof, token_stream, diagnostics)
}

fn expect_token(
    expected: &[TokenKind],
    token_stream: &mut TokenStream<Lexer>,
    diagnostics: &mut Diagnostics,
) -> Result<Token<TokenKind>, ParseError> {
    let current = token_stream.current().map(Clone::clone);
    match current {
        Ok(token) => {
            if expected.contains(&token.kind) {
                token_stream.next(diagnostics);
                Ok(token)
            } else {
                diagnostics.raise(MismatchedToken {
                    expected: expected.to_vec(),
                    found: token,
                });
                Err(ParseError)
            }
        },
        Err(_) => {
            token_stream.next(diagnostics);
            Err(ParseError)
        },
    }
}

fn try_expect_token(
    expected: &[TokenKind],
    token_stream: &mut TokenStream<Lexer>,
    diagnostics: &mut Diagnostics,
) -> Option<Token<TokenKind>> {
    let current = token_stream.current().map(Clone::clone);
    match current {
        Ok(token) => {
            if expected.contains(&token.kind) {
                token_stream.next(diagnostics);
                Some(token)
            } else {
                None
            }
        },
        Err(_) => {
            token_stream.next(diagnostics);
            None
        },
    }
}

fn parse_expr_with_end(
    end: TokenKind,
    token_stream: &mut TokenStream<Lexer>,
    diagnostics: &mut Diagnostics,
) -> Result<Expr, ParseError> {
    let mut curr_expr = None;
    let mut had_child_token = false;

    loop {
        match token_stream.current() {
            Ok(token) => match token.kind {
                kind if end == kind => break,
                TokenKind::Lambda => {
                    if let Ok(lambda) =
                        parse_lambda(end, token_stream, diagnostics)
                    {
                        curr_expr = Some(stack_exprs(curr_expr, lambda));
                    }
                },
                TokenKind::Ident => {
                    let next_expr = Expr {
                        span: token.span.clone(),
                        kind: ExprKind::Variable(token.span.clone()),
                    };
                    curr_expr = Some(stack_exprs(curr_expr, next_expr));
                    token_stream.next(diagnostics);
                },

                TokenKind::OpenParen => {
                    let open_paren = token.clone();
                    token_stream.next(diagnostics);
                    if let Ok(mut next_expr) = parse_expr_with_end(
                        TokenKind::CloseParen,
                        token_stream,
                        diagnostics,
                    ) {
                        let close_paren_res = try_expect_token(
                            &[TokenKind::CloseParen],
                            token_stream,
                            diagnostics,
                        );
                        next_expr.span = next_expr.span.join(&open_paren.span);
                        if let Some(close_paren) = close_paren_res {
                            next_expr.span =
                                next_expr.span.join(&close_paren.span);
                        } else {
                            let error =
                                UnmatchedOpenParen { span: open_paren.span };
                            diagnostics.raise(error);
                        }
                        curr_expr = Some(stack_exprs(curr_expr, next_expr));
                    }
                },

                TokenKind::Eof => break,

                TokenKind::CloseParen => {
                    let error =
                        UnmatchedCloseParen { span: token.span.clone() };
                    diagnostics.raise(error);
                    token_stream.next(diagnostics);
                },

                _ => {
                    let error = MismatchedToken {
                        expected: vec![
                            TokenKind::Lambda,
                            TokenKind::Ident,
                            TokenKind::OpenParen,
                            end,
                        ],
                        found: token.clone(),
                    };
                    diagnostics.raise(error);
                    token_stream.next(diagnostics);
                },
            },

            Err(_) => {
                token_stream.next(diagnostics);
            },
        }

        had_child_token = true;
    }

    match curr_expr {
        Some(expr) => Ok(expr),
        None => {
            if !had_child_token {
                let error = MismatchedToken {
                    expected: vec![
                        TokenKind::Lambda,
                        TokenKind::Ident,
                        TokenKind::OpenParen,
                    ],
                    found: token_stream.current().unwrap().clone(),
                };
                diagnostics.raise(error);
            }
            Err(ParseError)
        },
    }
}

fn parse_lambda(
    end: TokenKind,
    token_stream: &mut TokenStream<Lexer>,
    diagnostics: &mut Diagnostics,
) -> Result<Expr, ParseError> {
    let lambda_token = match token_stream.current() {
        Ok(token) => Ok(token.clone()),
        Err(_) => Err(ParseError),
    };
    token_stream.next(diagnostics);
    let parameter_result =
        expect_token(&[TokenKind::Ident], token_stream, diagnostics);
    let _ = expect_token(&[TokenKind::Dot], token_stream, diagnostics);
    let body = parse_expr_with_end(end, token_stream, diagnostics)?;
    let parameter = parameter_result?;
    let full_span = match lambda_token {
        Ok(lambda) => lambda.span.join(&body.span),
        Err(_) => parameter.span.join(&body.span),
    };

    Ok(Expr {
        span: full_span,
        kind: ExprKind::Lambda {
            parameter: parameter.span,
            body: Box::new(body),
        },
    })
}

fn stack_exprs(left: Option<Expr>, right: Expr) -> Expr {
    match left {
        Some(function) => Expr {
            span: function.span.join(&right.span),
            kind: ExprKind::Application {
                function: Box::new(function),
                argument: Box::new(right),
            },
        },

        None => right,
    }
}

fn write_indent<W>(level: u32, target: &mut W) -> fmt::Result
where
    W: fmt::Write + ?Sized,
{
    for _ in 0 .. level {
        write!(target, "    ")?;
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct DisplayExpr<'expr> {
    expr: &'expr Expr,
    level: u32,
}

impl<'expr> fmt::Display for DisplayExpr<'expr> {
    fn fmt(&self, fmtr: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_indent(self.level, fmtr)?;
        write!(fmtr, "[{}]\n", self.expr.span)?;
        write!(
            fmtr,
            "{}",
            DisplayExprKind { kind: &self.expr.kind, level: self.level }
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct DisplayExprKind<'expr> {
    kind: &'expr ExprKind,
    level: u32,
}

impl<'expr> fmt::Display for DisplayExprKind<'expr> {
    fn fmt(&self, fmtr: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ExprKind::Variable(name) => {
                write_indent(self.level, fmtr)?;
                write!(fmtr, "{}\n", name.as_str())
            },
            ExprKind::Application { function, argument } => {
                let fun_needs_parens =
                    matches!(&function.kind, ExprKind::Lambda { .. });
                let arg_needs_parens = matches!(
                    &argument.kind,
                    ExprKind::Application { .. } | ExprKind::Lambda { .. }
                );

                if fun_needs_parens {
                    write_indent(self.level + 1, fmtr)?;
                    write!(fmtr, "(\n")?;
                    write!(
                        fmtr,
                        "{}",
                        DisplayExpr { expr: function, level: self.level + 2 }
                    )?;
                    write_indent(self.level + 1, fmtr)?;
                    write!(fmtr, ")\n")?;
                } else {
                    write!(
                        fmtr,
                        "{}",
                        DisplayExpr { expr: function, level: self.level + 1 }
                    )?;
                }

                if arg_needs_parens {
                    write_indent(self.level + 1, fmtr)?;
                    write!(fmtr, "(\n")?;
                    write!(
                        fmtr,
                        "{}",
                        DisplayExpr { expr: argument, level: self.level + 2 }
                    )?;
                    write_indent(self.level + 1, fmtr)?;
                    write!(fmtr, ")\n")?;
                } else {
                    write!(
                        fmtr,
                        "{}",
                        DisplayExpr { expr: argument, level: self.level + 1 }
                    )?;
                }

                Ok(())
            },

            ExprKind::Lambda { parameter, body } => {
                write_indent(self.level, fmtr)?;
                write!(
                    fmtr,
                    "\\{}.\n{}",
                    parameter.as_str(),
                    DisplayExpr { expr: body, level: self.level + 1 }
                )
            },
        }
    }
}

fn show_help() -> ! {
    eprintln!("Parses and show source code with AST nodes spans/locations");
    eprintln!("Usage:");
    eprintln!("    lambda --stdin                Reads source from stdin");
    eprintln!("    lambda -f FILEPATH            Reads source from file");
    eprintln!("    lambda -h                     Shows this message and exits");
    process::exit(1);
}

fn main() {
    let mut args = env::args_os();
    args.next();

    let source_contents = match args.next() {
        Some(arg) if arg == "--stdin" => {
            let mut buf = String::new();
            match io::Read::read_to_string(&mut io::stdin(), &mut buf) {
                Ok(_) => buf,
                Err(error) => {
                    eprintln!("stdin: {}", error);
                    process::exit(1);
                },
            }
        },

        Some(arg) if arg == "-f" => match args.next() {
            Some(path_str) => match fs::read_to_string(&path_str) {
                Ok(buf) => buf,
                Err(error) => {
                    eprintln!("{}: {}", Path::new(&path_str).display(), error);
                    process::exit(1);
                },
            },
            None => show_help(),
        },

        Some(arg) if arg == "-h" => show_help(),

        _ => show_help(),
    };

    let source = Source::new("main.lam", source_contents);
    let mut diagnostics = Diagnostics::new();
    let mut token_stream = TokenStream::new(&source, Lexer, &mut diagnostics);

    let parse_result = parse_expr(&mut token_stream, &mut diagnostics);

    for diagnostic in diagnostics {
        eprint!("{}", diagnostic);
        if let Some(span) = diagnostic.primary_span() {
            eprint!(", {}", span);
        }
        eprintln!();
    }

    if let Ok(expr) = parse_result {
        println!("{}", expr);
    }
}
