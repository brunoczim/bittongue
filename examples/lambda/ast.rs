//! Module with AST items (Abstract Syntax Tree), basically exports lambda
//! calculus expressions.

use bittongue::source::Span;
use std::fmt;

/// A lambda calculus expression.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Expr {
    /// Span of the whole expression.
    pub span: Span,
    /// The kind of this expression.
    pub kind: ExprKind,
}

impl fmt::Display for Expr {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}", DisplayExpr { expr: self, level: 0 })
    }
}

/// Expression kind, i.e. the actual node in the AST.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExprKind {
    /// Variable node, e.g. `foo`.
    Variable(Span),
    /// Application node, e.g. `f x`.
    Application { function: Box<Expr>, argument: Box<Expr> },
    /// Lambda node, e.g. `\x. e`.
    Lambda { parameter: Span, body: Box<Expr> },
}

impl fmt::Display for ExprKind {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}", DisplayExprKind { kind: self, level: 0 })
    }
}

/// Writes indentation to a formatter.
fn write_indent<W>(level: u32, target: &mut W) -> fmt::Result
where
    W: fmt::Write + ?Sized,
{
    for _ in 0 .. level {
        write!(target, "    ")?;
    }
    Ok(())
}

/// Displays an expression with the given level of indentation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct DisplayExpr<'expr> {
    /// Expression being displayed.
    expr: &'expr Expr,
    /// Level of indentation.
    level: u32,
}

impl<'expr> fmt::Display for DisplayExpr<'expr> {
    fn fmt(&self, fmtr: &mut fmt::Formatter<'_>) -> fmt::Result {
        // indentation for the span.
        write_indent(self.level, fmtr)?;
        // writes the span
        write!(fmtr, "[{}]\n", self.expr.span)?;
        // delegates further rendering to the expression's kind.
        write!(
            fmtr,
            "{}",
            DisplayExprKind { kind: &self.expr.kind, level: self.level }
        )?;
        Ok(())
    }
}

/// Displays an expression *kind* with the given level of indentation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct DisplayExprKind<'expr> {
    /// Expression kind being displayed.
    kind: &'expr ExprKind,
    /// Level of indentation.
    level: u32,
}

impl<'expr> fmt::Display for DisplayExprKind<'expr> {
    fn fmt(&self, fmtr: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            // Renders a variable.
            ExprKind::Variable(name) => {
                // Indentation for the variable name.
                write_indent(self.level, fmtr)?;
                write!(fmtr, "{}\n", name.as_str())
            },

            // Renders an application.
            ExprKind::Application { function, argument } => {
                // Function needs parenthesis if it is a lambda node.
                let fun_needs_parens =
                    matches!(&function.kind, ExprKind::Lambda { .. });
                // Argument needs parenthesis if it is a lambda node or an
                // application node.
                let arg_needs_parens = matches!(
                    &argument.kind,
                    ExprKind::Application { .. } | ExprKind::Lambda { .. }
                );

                if fun_needs_parens {
                    // Advanced indentation for parenthesis.
                    write_indent(self.level + 1, fmtr)?;
                    write!(fmtr, "(\n")?;
                    // Renders function with two levels of indentation ahead.
                    write!(
                        fmtr,
                        "{}",
                        DisplayExpr { expr: function, level: self.level + 2 }
                    )?;
                    // Closes parenthesis.
                    write_indent(self.level + 1, fmtr)?;
                    write!(fmtr, ")\n")?;
                } else {
                    // Renders function without parenthesis.
                    write!(
                        fmtr,
                        "{}",
                        DisplayExpr { expr: function, level: self.level + 1 }
                    )?;
                }

                if arg_needs_parens {
                    // Advanced indentation for parenthesis.
                    write_indent(self.level + 1, fmtr)?;
                    write!(fmtr, "(\n")?;
                    // Renders function with two levels of indentation ahead.
                    write!(
                        fmtr,
                        "{}",
                        DisplayExpr { expr: argument, level: self.level + 2 }
                    )?;
                    // Closes parenthesis.
                    write_indent(self.level + 1, fmtr)?;
                    write!(fmtr, ")\n")?;
                } else {
                    // Renders argument without parenthesis.
                    write!(
                        fmtr,
                        "{}",
                        DisplayExpr { expr: argument, level: self.level + 1 }
                    )?;
                }

                Ok(())
            },

            // Render a lambda node.
            ExprKind::Lambda { parameter, body } => {
                // Indentation for lambda parameter.
                write_indent(self.level, fmtr)?;
                // Lambda parameter in this line, body in the next line with
                // increased indentation.
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
