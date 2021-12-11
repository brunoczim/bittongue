use bittongue::source::Span;
use std::fmt;

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
