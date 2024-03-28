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

            Statement::AssignToArray { ref vars, ref rhs } => RcDoc::text("let [")
                .append(RcDoc::intersperse(
                    vars.iter().map(|arg| RcDoc::text(arg)),
                    RcDoc::text(", "),
                ))
                .append(RcDoc::text("] = "))
                .append(rhs.to_doc())
                .append(RcDoc::text(";")),

            Statement::AssignFromArray { ref vars, ref rhs } => RcDoc::text("let [")
                .append(RcDoc::intersperse(
                    vars.iter().map(|arg| RcDoc::text(arg)),
                    RcDoc::text(", "),
                ))
                .append(RcDoc::text("] = ["))
                .append(RcDoc::intersperse(
                    rhs.iter().map(|arg| arg.to_doc()),
                    RcDoc::text(", "),
                ))
                .append(RcDoc::text("];")),

            Statement::Sequence(ref stmts) => {
                RcDoc::intersperse(stmts.iter().map(|stmt| stmt.to_doc()), RcDoc::line())
            }

            Statement::Return(ref expr) => expr.to_doc(),

            Statement::Empty => RcDoc::nil(),
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
            Expr::Function { ref name, ref args } => {
                let mut doc = name.to_doc().append(RcDoc::text("("));
                for (i, arg) in args.iter().enumerate() {
                    doc = doc.append(arg.to_doc());
                    if i < args.len() - 1 {
                        doc = doc.append(RcDoc::text(", "));
                    }
                }
                doc.append(RcDoc::text(")"))
            }
            Expr::Variable(ref s) => RcDoc::text(s),
            Expr::TernaryOp(ref cond, ref if_true, ref if_false) => RcDoc::text("if ")
                .append(cond.to_doc())
                .append(RcDoc::text(" { "))
                .append(if_true.to_doc())
                .append(RcDoc::text(" } else { "))
                .append(if_false.to_doc())
                .append(RcDoc::text(" }")),
            Expr::Assign(ref assign) => match assign {
                AssignExpr::Inc(ref s) => RcDoc::text("let ")
                    .append(RcDoc::as_string(s))
                    .append(RcDoc::text(" = "))
                    .append(RcDoc::as_string(s))
                    .append(RcDoc::text(" + 1;")),
                AssignExpr::Dec(ref s) => RcDoc::text("let ")
                    .append(RcDoc::as_string(s))
                    .append(RcDoc::text(" = "))
                    .append(RcDoc::as_string(s))
                    .append(RcDoc::text(" - 1;")),
            },
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
            FunctionName::Exp => RcDoc::text("exp"),
            FunctionName::Exp2 => RcDoc::text("exp2"),
            FunctionName::Log => RcDoc::text("log"),
            FunctionName::Log2 => RcDoc::text("log2"),
            FunctionName::Pow => RcDoc::text("pow"),
            FunctionName::Sqrt => RcDoc::text("sqrt"),
            FunctionName::Abs => RcDoc::text("abs"),
            FunctionName::Sign => RcDoc::text("signum"),
            FunctionName::Floor => RcDoc::text("floor"),
            FunctionName::Ceil => RcDoc::text("ceil"),
            FunctionName::Trunc => RcDoc::text("trunc"),
            FunctionName::Fract => RcDoc::text("fract"),
            FunctionName::Mod => RcDoc::text("modulo"),
            FunctionName::Min => RcDoc::text("min"),
            FunctionName::Max => RcDoc::text("max"),
            FunctionName::Clamp => RcDoc::text("clamp"),
            FunctionName::Mix => RcDoc::text("mix"),
            FunctionName::Smoothstep => RcDoc::text("smoothstep"),
            FunctionName::Length => RcDoc::text("length"),
            FunctionName::Distance => RcDoc::text("distance"),
            FunctionName::Dot => RcDoc::text("dot"),
            FunctionName::Union => RcDoc::text("union"),
            FunctionName::Intersect => RcDoc::text("intersect"),
            FunctionName::Cross => RcDoc::text("cross"),
            FunctionName::Normalize => RcDoc::text("normalize"),
            FunctionName::RoundMin => RcDoc::text("smooth_min"),
            FunctionName::RoundMax => RcDoc::text("smooth_max"),
            FunctionName::SmoothAbs => RcDoc::text("smooth_abs"),
            FunctionName::PolySmoothAbs => RcDoc::text("poly_smooth_abs"),
            FunctionName::SmoothClamp => RcDoc::text("smooth_clamp"),
            FunctionName::PolySmoothClamp => RcDoc::text("poly_smooth_clamp"),
            FunctionName::ValueNoise => RcDoc::text("value_noise"),
            FunctionName::Torus => RcDoc::text("torus"),
            FunctionName::Box2 => RcDoc::text("box2"),
            FunctionName::Box3 => RcDoc::text("box3"),
            FunctionName::Rot0 => RcDoc::text("rot0"),
            FunctionName::Rot1 => RcDoc::text("rot1"),
            FunctionName::Rot => RcDoc::text("rot"),
            FunctionName::Triangle => RcDoc::text("triangle"),
            FunctionName::Corner => RcDoc::text("corner"),
            FunctionName::FakeSine => RcDoc::text("fake_sine"),
            FunctionName::Hash => RcDoc::text("hash"),
            FunctionName::AddMul => RcDoc::text("add_mul"),
            FunctionName::Round => RcDoc::text("round"),
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::pratt::parse;

    #[test]
    fn show() {
        // let mut i = "s=10; @1{a=sin(y),b=sin(x),c=sin(z),d=x,e=s+1,} SM(a,b,c,d,e)-5";
        let mut i = "U(L(x+28,y-10,z+8)-12, don(x-cl(x,-15,15),y-18,z-20,10,3), bx3(x-20,y-20,z+20,8)-10, L(x+3,y-16)-4)";
        let ast = parse(&mut i);
        dbg!(ast.to_pretty(80));
    }

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

        let mut i = "x += y / 2; s = U(x,y,z)";
        let ast = parse(&mut i);
        assert_eq!(
            ast.to_pretty(80),
            "x = x + y / 2f32;\nlet s = union(x, y, z);"
        );
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
        assert_eq!(ast.to_pretty(80), "let s = sin(1f32);");

        let ast = Statement::Assign {
            var: "t".to_string(),
            rhs: Box::new(Expr::Function {
                name: FunctionName::Atan2,
                args: vec![Expr::Number(1.0), Expr::Number(2.0)],
            }),
        };
        assert_eq!(ast.to_pretty(80), "let t = atan2(1f32, 2f32);");
    }

    #[test]
    fn ternary() {
        let ast = Statement::Assign {
            var: "s".to_string(),
            rhs: Box::new(Expr::TernaryOp(
                Box::new(Expr::Number(1.0)),
                Box::new(Expr::Number(2.0)),
                Box::new(Expr::Number(3.0)),
            )),
        };
        assert_eq!(ast.to_pretty(80), "let s = if 1f32 { 2f32 } else { 3f32 };");
    }

    #[test]
    fn assign_to() {
        let ast = Statement::AssignToArray {
            vars: vec!["s".to_string(), "t".to_string()],
            rhs: Box::new(Expr::Number(1.0)),
        };
        assert_eq!(ast.to_pretty(80), "let [s, t] = 1f32;");
    }

    #[test]
    fn assign_from() {
        let ast = Statement::AssignFromArray {
            vars: vec!["s".to_string(), "t".to_string()],
            rhs: vec![Expr::Number(1.0), Expr::Number(2.0)],
        };
        assert_eq!(ast.to_pretty(80), "let [s, t] = [1f32, 2f32];");
    }

    #[test]
    fn assign_expr() {
        let ast = Statement::Return(Box::new(Expr::Assign(AssignExpr::Inc("s".to_string()))));
        assert_eq!(ast.to_pretty(80), "let s = s + 1;");
    }
}
