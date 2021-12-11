//! This program implements a parser of lambda calculus. [`main`] function will
//! read source from a file, parse it, show errors possibly found, and print the
//! resulting expression with span/location of each node.

pub mod token;
pub mod lexer;
pub mod ast;
pub mod parser;

use crate::{lexer::Lexer, parser::parse_expr};
use bittongue::{
    diagnostic::{Diagnostic, Diagnostics},
    lexer::TokenStream,
    source::Source,
};
use std::{env, fs, io, path::Path, process};

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
