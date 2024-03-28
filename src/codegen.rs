use std::collections::VecDeque;

use crate::ast::{AssignExpr, BinOp, Expr, FunctionName, Statement};
use pretty::RcDoc;

impl Statement {
    pub fn to_doc(&self) -> RcDoc<()> {
        match *self {
            Statement::Assign { ref var, ref rhs } => if var != "x" && var != "y" && var != "z" {
                RcDoc::text("let ")
            } else {
                RcDoc::text("")
            }
            .append(RcDoc::as_string(var))
            .append(RcDoc::text(" = "))
            .append(rhs.to_doc())
            .append(RcDoc::text(";")),

            Statement::Sequence(ref stmts) => {
                RcDoc::intersperse(stmts.iter().map(|stmt| stmt.to_doc()), RcDoc::line())
            }

            Statement::Return(ref expr) => expr.to_doc(),

            _ => RcDoc::nil(),
        }
    }
    pub fn to_pretty(&self, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc().render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

impl Expr {
    pub fn to_doc(&self) -> RcDoc<()> {
        match *self {
            Expr::Number(n) => RcDoc::as_string(format!("{}f32", n)),
            Expr::BinaryOp(ref op) => op.to_doc(),
            Expr::Negate(ref e) => RcDoc::text("-").append(e.to_doc()),
            Expr::Variable(ref s) => RcDoc::text(s).append(RcDoc::space()),
            Expr::Function { ref name, ref args } => args[0]
                .to_doc()
                .append(RcDoc::text("."))
                .append(name.to_doc())
                .append(RcDoc::text("("))
                .append(RcDoc::intersperse(
                    args[1..].iter().map(|arg| arg.to_doc()),
                    RcDoc::text(", "),
                ))
                .append(RcDoc::text(")")),
            _ => RcDoc::text("Expression"),
        }
    }
}

impl BinOp {
    pub fn to_doc(&self) -> RcDoc<()> {
        match *self {
            BinOp::Add(ref lhs, ref rhs) => {
                lhs.to_doc().append(RcDoc::text(" + ").append(rhs.to_doc()))
            }
            BinOp::Sub(ref lhs, ref rhs) => {
                lhs.to_doc().append(RcDoc::text(" - ").append(rhs.to_doc()))
            }
            BinOp::Mul(ref lhs, ref rhs) => {
                lhs.to_doc().append(RcDoc::text(" * ").append(rhs.to_doc()))
            }
            BinOp::Div(ref lhs, ref rhs) => {
                lhs.to_doc().append(RcDoc::text(" / ").append(rhs.to_doc()))
            }
            BinOp::Eq(ref lhs, ref rhs) => lhs
                .to_doc()
                .append(RcDoc::text(" == ").append(rhs.to_doc())),
            BinOp::NotEq(ref lhs, ref rhs) => lhs
                .to_doc()
                .append(RcDoc::text(" != ").append(rhs.to_doc())),
            BinOp::Greater(ref lhs, ref rhs) => {
                lhs.to_doc().append(RcDoc::text(" > ").append(rhs.to_doc()))
            }
            BinOp::GreaterEq(ref lhs, ref rhs) => lhs
                .to_doc()
                .append(RcDoc::text(" >= ").append(rhs.to_doc())),
            BinOp::Less(ref lhs, ref rhs) => {
                lhs.to_doc().append(RcDoc::text(" < ").append(rhs.to_doc()))
            }
            BinOp::LessEq(ref lhs, ref rhs) => lhs
                .to_doc()
                .append(RcDoc::text(" <= ").append(rhs.to_doc())),
            BinOp::And(ref lhs, ref rhs) => lhs
                .to_doc()
                .append(RcDoc::text(" && ").append(rhs.to_doc())),
            BinOp::Or(ref lhs, ref rhs) => lhs
                .to_doc()
                .append(RcDoc::text(" || ").append(rhs.to_doc())),
            BinOp::Pow(ref lhs, ref rhs) => lhs
                .to_doc()
                .append(RcDoc::text(".powf(").append(rhs.to_doc()))
                .append(RcDoc::text(")")),
        }
    }
}

impl FunctionName {
    pub fn to_doc(&self) -> RcDoc<()> {
        match *self {
            FunctionName::Sin => RcDoc::text("sin"),
            FunctionName::Cos => RcDoc::text("cos"),
            FunctionName::Acos => RcDoc::text("acos"),
            FunctionName::Asin => RcDoc::text("asin"),
            FunctionName::Tan => RcDoc::text("tan"),
            FunctionName::Atan => RcDoc::text("atan"),
            FunctionName::Atan2 => RcDoc::text("atan2"),
            FunctionName::Sinh => RcDoc::text("sinh"),
            FunctionName::Cosh => RcDoc::text("cosh"),
            FunctionName::Tanh => RcDoc::text("tanh"),
            FunctionName::Asinh => RcDoc::text("asinh"),
            FunctionName::Acosh => RcDoc::text("acosh"),
            FunctionName::Atanh => RcDoc::text("atanh"),
            _ => RcDoc::nil(),
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn assign_variable() {
        let ast = Statement::Assign {
            var: "s".to_string(),
            rhs: Box::new(Expr::Number(1.0)),
        };
        assert_eq!(ast.to_pretty(80), "let s = 1f32;");
    }

    #[test]
    fn sequence() {
        let mut stmts = Vec::new();
        stmts.push(Statement::Assign {
            var: "x".to_string(),
            rhs: Box::new(Expr::Number(1.0)),
        });
        stmts.push(Statement::Assign {
            var: "s".to_string(),
            rhs: Box::new(Expr::Number(2.3)),
        });
        let ast = Statement::Sequence(stmts);
        assert_eq!(ast.to_pretty(80), "x = 1f32;\nlet s = 2.3f32;");
    }

    #[test]
    fn binop() {
        let ast = Statement::Assign {
            var: "s".to_string(),
            rhs: Box::new(Expr::BinaryOp(BinOp::GreaterEq(
                Box::new(Expr::Number(1.0)),
                Box::new(Expr::Number(2.0)),
            ))),
        };
        assert_eq!(ast.to_pretty(80), "let s = 1f32 >= 2f32;");

        let ast = Statement::Assign {
            var: "t".to_string(),
            rhs: Box::new(Expr::BinaryOp(BinOp::Pow(
                Box::new(Expr::Number(1.0)),
                Box::new(Expr::Number(2.0)),
            ))),
        };
        assert_eq!(ast.to_pretty(80), "let t = 1f32.powf(2f32);");
    }

    #[test]
    fn func() {
        let ast = Statement::Assign {
            var: "s".to_string(),
            rhs: Box::new(Expr::Function {
                name: FunctionName::Sin,
                args: vec![Expr::Number(1.0)],
            }),
        };
        assert_eq!(ast.to_pretty(80), "let s = 1f32.sin();");

        let ast = Statement::Assign {
            var: "t".to_string(),
            rhs: Box::new(Expr::Function {
                name: FunctionName::Atan2,
                args: vec![Expr::Number(1.0), Expr::Number(2.0)],
            }),
        };
        assert_eq!(ast.to_pretty(80), "let t = 1f32.atan2(2f32);");
    }
}
