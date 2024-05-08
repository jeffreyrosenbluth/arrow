use crate::ast::{AssignExpr, BinOp, Expr, FunctionName, Statement};
use crate::core::{fbm_perlin, fbm_value};
use crate::functions::*;
use pretty::RcDoc;
use rhai::{Array, Engine};

fn length2(x: f32, y: f32) -> f32 {
    (x * x + y * y).sqrt()
}

fn length3(x: f32, y: f32, z: f32) -> f32 {
    (x * x + y * y + z * z).sqrt()
}

fn distance3(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> f32 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2) + (z1 - z2).powi(2)).sqrt()
}

fn distance2(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}

fn dot2(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    x1 * x2 + y1 * y2
}

fn dot3(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> f32 {
    x1 * x2 + y1 * y2 + z1 * z2
}

fn normalize3(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    let len = length3(x, y, z);
    (x / len, y / len, z / len)
}

fn normalize2(x: f32, y: f32) -> (f32, f32) {
    let len = length2(x, y);
    (x / len, y / len)
}

fn add_mul3(x: f32, y: f32, z: f32, a: f32, b: f32, c: f32, t: f32) -> (f32, f32, f32) {
    (x + a * t, y + b * t, z + c * t)
}

fn add_mul2(x: f32, y: f32, a: f32, b: f32, t: f32) -> (f32, f32) {
    (x + a * t, y + b * t)
}

fn value_noise0(x: f32, y: f32, z: f32, s: f32, i: f32, o: f32) -> f32 {
    fbm_value(x, y, z, s, i, o as u32)
}

fn value_noise1(x: f32, y: f32, s: f32, i: f32) -> f32 {
    fbm_value(x, y, 0.0, s, i, 1u32)
}

fn perlin_noise0(x: f32, y: f32, z: f32, s: f32, i: f32, o: f32) -> f32 {
    fbm_perlin(x, y, z, s, i, o as u32)
}

fn perlin_noise1(x: f32, y: f32, s: f32, i: f32) -> f32 {
    fbm_perlin(x, y, 0.0, s, i, 1u32)
}

fn box3(x: f32, y: f32, z: f32, a: f32, b: f32, c: f32) -> f32 {
    let qx = abs(x) - a;
    let qy = abs(y) - b;
    let qz = abs(z) - c;
    min(max(qy, max(qz, qx)), 0.0) + length3(max(qx, 0.0), max(qy, 0.0), max(qz, 0.0))
}

fn box3a(x: f32, y: f32, z: f32, a: f32) -> f32 {
    let qx = abs(x) - a;
    let qy = abs(y) - a;
    let qz = abs(z) - a;
    min(max(qy, max(qz, qx)), 0.0) + length3(max(qx, 0.0), max(qy, 0.0), max(qz, 0.0))
}

fn box2(x: f32, y: f32, a: f32, b: f32) -> f32 {
    let qx = abs(x) - a;
    let qy = abs(y) - b;
    min(max(qy, qx), 0.0) + length2(max(qx, 0.0), max(qy, 0.0))
}

fn box2a(x: f32, y: f32, a: f32) -> f32 {
    let qx = abs(x) - a;
    let qy = abs(y) - a;
    min(max(qy, qx), 0.0) + length2(max(qx, 0.0), max(qy, 0.0))
}

fn union_array(arr: Array) -> f32 {
    let mut d = f32::INFINITY;
    let vec: Vec<f32> = arr
        .into_iter()
        .map(|value| value.as_float().unwrap())
        .collect();
    for x in vec {
        d = min(d, x);
    }
    d
}

fn round_min_array(arr: Array) -> f32 {
    let vec: Vec<f32> = arr
        .into_iter()
        .map(|value| value.as_float().unwrap())
        .collect();
    round_min(vec)
}

fn round_max_array(arr: Array) -> f32 {
    let vec: Vec<f32> = arr
        .into_iter()
        .map(|value| value.as_float().unwrap())
        .collect();
    round_max(vec)
}

fn intersect_array(arr: Array) -> f32 {
    let vec: Vec<f32> = arr
        .into_iter()
        .map(|value| value.as_float().unwrap())
        .collect();
    intersect(vec)
}

pub fn base_engine() -> Engine {
    let mut engine = Engine::new();
    engine.register_fn("log2", log2);
    engine.register_fn("fake_sine", fake_sine);
    engine.register_fn("modulo", modulo);
    engine.register_fn("minf", min);
    engine.register_fn("maxf", max);
    engine.register_fn("clamp", clamp);
    engine.register_fn("mix", mix);
    engine.register_fn("smoothstep", smoothstep);
    engine.register_fn("length", length2);
    engine.register_fn("length", length3);
    engine.register_fn("distance", distance2);
    engine.register_fn("distance", distance3);
    engine.register_fn("dot", dot2);
    engine.register_fn("dot", dot3);
    engine.register_fn("cross", cross);
    engine.register_fn("normalize", normalize3);
    engine.register_fn("normalize", normalize2);
    engine.register_fn("union", union_array);
    engine.register_fn("round_min", round_min_array);
    engine.register_fn("round_max", round_max_array);
    engine.register_fn("intersect", intersect_array);
    engine.register_fn("add_mul", add_mul2);
    engine.register_fn("add_mul", add_mul3);
    engine.register_fn("smooth_min", smooth_min);
    engine.register_fn("smooth_max", smooth_max);
    engine.register_fn("poly_smooth_abs", poly_smooth_abs);
    engine.register_fn("smooth_clamp", smooth_clamp);
    engine.register_fn("poly_smooth_clamp", poly_smooth_clamp);
    engine.register_fn("smooth_abs", smooth_abs);
    engine.register_fn("poly_smooth_abs", poly_smooth_abs);
    engine.register_fn("torus", torus);
    engine.register_fn("value_noise", value_noise0);
    engine.register_fn("value_noise", value_noise1);
    engine.register_fn("perlin_noise", perlin_noise0);
    engine.register_fn("perlin_noise", perlin_noise1);
    engine.register_fn("box3", box3);
    engine.register_fn("box3", box3a);
    engine.register_fn("box2", box2);
    engine.register_fn("box2", box2a);
    engine.register_fn("rot", rot);
    engine.register_fn("rot0", rot0);
    engine.register_fn("triangle", triangle);
    engine.register_fn("corner", corner);
    engine.register_fn("hash", hash);
    engine
}

pub fn generate_code(ast: &Statement, a0: f32, a1: f32) -> String {
    let doc = RcDoc::text("fn signed_distance_function(x, y, z) {")
        .append(RcDoc::line())
        .append(RcDoc::text(format!("let a0 = {};", a0)))
        .append(RcDoc::line())
        .append(RcDoc::text(format!("let a1 = {};", a1)))
        .append(RcDoc::line())
        .append(ast.to_doc())
        .append(RcDoc::line())
        .nest(4)
        .append(RcDoc::text("}"));

    let mut w = Vec::new();
    doc.render(100, &mut w).unwrap();
    String::from_utf8(w).unwrap()
}

impl Statement {
    pub fn to_doc(&self) -> RcDoc<()> {
        match *self {
            Statement::Assign { ref var, ref rhs } => RcDoc::text("let ")
                .append(RcDoc::as_string(var))
                .append(RcDoc::text(" = "))
                .append(rhs.to_doc(0))
                .append(RcDoc::text(";")),

            Statement::AssignToArray { ref vars, ref rhs } => RcDoc::text("let [")
                .append(RcDoc::intersperse(
                    vars.iter().map(|arg| RcDoc::text(arg)),
                    RcDoc::text(", "),
                ))
                .append(RcDoc::text("] = "))
                .append(rhs.to_doc(0))
                .append(RcDoc::text(";")),

            Statement::AssignFromArray { ref vars, ref rhs } => RcDoc::text("let [")
                .append(RcDoc::intersperse(
                    vars.iter().map(|arg| RcDoc::text(arg)),
                    RcDoc::text(", "),
                ))
                .append(RcDoc::text("] = ["))
                .append(RcDoc::intersperse(
                    rhs.iter().map(|arg| arg.to_doc(0)),
                    RcDoc::text(", "),
                ))
                .append(RcDoc::text("];")),

            Statement::Sequence(ref stmts) => {
                RcDoc::intersperse(stmts.iter().map(|stmt| stmt.to_doc()), RcDoc::line())
            }

            Statement::Return(ref expr) => expr.to_doc(0),

            Statement::Empty => RcDoc::nil(),
        }
    }

    pub fn to_pretty(&self, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc().render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

fn get_precedence(binop: &BinOp) -> u8 {
    match *binop {
        BinOp::Add(_, _) | BinOp::Sub(_, _) => 1,
        BinOp::Mul(_, _) | BinOp::Div(_, _) => 2,
        BinOp::Eq(_, _)
        | BinOp::NotEq(_, _)
        | BinOp::Greater(_, _)
        | BinOp::GreaterEq(_, _)
        | BinOp::Less(_, _)
        | BinOp::LessEq(_, _) => 3,
        BinOp::And(_, _) | BinOp::Or(_, _) => 4,
        BinOp::Pow(_, _) => 5,
    }
}

impl Expr {
    pub fn to_doc(&self, precedence: u8) -> RcDoc<()> {
        match *self {
            Expr::Number(n) => RcDoc::as_string(format!("{:.4}", n)),
            Expr::BinaryOp(ref op) => op.to_doc(precedence),
            Expr::Negate(ref e) => RcDoc::text("-").append(e.to_doc(precedence)),
            Expr::Function { ref name, ref args }
                if *name == FunctionName::Union
                    || *name == FunctionName::RoundMin
                    || *name == FunctionName::Intersect
                    || *name == FunctionName::RoundMax =>
            {
                let mut doc = name.to_doc().append(RcDoc::text("(["));
                for (i, arg) in args.iter().enumerate() {
                    doc = doc.append(arg.to_doc(0));
                    if i < args.len() - 1 {
                        doc = doc.append(RcDoc::text(", "));
                    }
                }
                doc.append(RcDoc::text("])"))
            }
            Expr::Function { ref name, ref args } => {
                let mut doc = name.to_doc().append(RcDoc::text("("));
                for (i, arg) in args.iter().enumerate() {
                    doc = doc.append(arg.to_doc(0));
                    if i < args.len() - 1 {
                        doc = doc.append(RcDoc::text(", "));
                    }
                }
                if *name == FunctionName::Rot0 {
                    doc = doc.append(RcDoc::text(", a0"));
                } else if *name == FunctionName::Rot1 {
                    doc = doc.append(RcDoc::text(", a1"));
                }
                doc.append(RcDoc::text(")"))
            }
            Expr::Variable(ref s) => RcDoc::text(s),
            Expr::TernaryOp(ref cond, ref if_true, ref if_false) => RcDoc::text("if ")
                .append(cond.to_doc(precedence))
                .append(RcDoc::text(" { "))
                .append(if_true.to_doc(precedence))
                .append(RcDoc::text(" } else { "))
                .append(if_false.to_doc(precedence))
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
    pub fn to_doc(&self, precedence: u8) -> RcDoc<()> {
        let op_prec = get_precedence(self);
        let (left, right) = if precedence > op_prec {
            (RcDoc::text("("), RcDoc::text(")"))
        } else {
            (RcDoc::nil(), RcDoc::nil())
        };
        match *self {
            BinOp::Add(ref lhs, ref rhs) => {
                let lhs = lhs.to_doc(op_prec);
                let rhs = rhs.to_doc(op_prec);
                left.append(lhs)
                    .append(RcDoc::text(" + "))
                    .append(rhs)
                    .append(right)
            }
            BinOp::Sub(ref lhs, ref rhs) => {
                let lhs = lhs.to_doc(op_prec);
                let rhs = rhs.to_doc(op_prec);
                left.append(lhs)
                    .append(RcDoc::text(" - "))
                    .append(rhs)
                    .append(right)
            }
            BinOp::Mul(ref lhs, ref rhs) => {
                let lhs = lhs.to_doc(op_prec);
                let rhs = rhs.to_doc(op_prec);
                left.append(lhs)
                    .append(RcDoc::text(" * "))
                    .append(rhs)
                    .append(right)
            }
            BinOp::Div(ref lhs, ref rhs) => {
                let lhs = lhs.to_doc(op_prec);
                let rhs = rhs.to_doc(op_prec);
                left.append(lhs)
                    .append(RcDoc::text(" / "))
                    .append(rhs)
                    .append(right)
            }
            BinOp::Eq(ref lhs, ref rhs) => {
                let lhs = lhs.to_doc(op_prec);
                let rhs = rhs.to_doc(op_prec);
                left.append(lhs)
                    .append(RcDoc::text(" == "))
                    .append(rhs)
                    .append(right)
            }
            BinOp::NotEq(ref lhs, ref rhs) => {
                let lhs = lhs.to_doc(op_prec);
                let rhs = rhs.to_doc(op_prec);
                left.append(lhs)
                    .append(RcDoc::text(" != "))
                    .append(rhs)
                    .append(right)
            }
            BinOp::Greater(ref lhs, ref rhs) => {
                let lhs = lhs.to_doc(op_prec);
                let rhs = rhs.to_doc(op_prec);
                left.append(lhs)
                    .append(RcDoc::text(" > "))
                    .append(rhs)
                    .append(right)
            }
            BinOp::GreaterEq(ref lhs, ref rhs) => {
                let lhs = lhs.to_doc(op_prec);
                let rhs = rhs.to_doc(op_prec);
                left.append(lhs)
                    .append(RcDoc::text(" >= "))
                    .append(rhs)
                    .append(right)
            }
            BinOp::Less(ref lhs, ref rhs) => {
                let lhs = lhs.to_doc(op_prec);
                let rhs = rhs.to_doc(op_prec);
                left.append(lhs)
                    .append(RcDoc::text(" < "))
                    .append(rhs)
                    .append(right)
            }
            BinOp::LessEq(ref lhs, ref rhs) => {
                let lhs = lhs.to_doc(op_prec);
                let rhs = rhs.to_doc(op_prec);
                left.append(lhs)
                    .append(RcDoc::text(" <= "))
                    .append(rhs)
                    .append(right)
            }
            BinOp::And(ref lhs, ref rhs) => {
                let lhs = lhs.to_doc(op_prec);
                let rhs = rhs.to_doc(op_prec);
                left.append(lhs)
                    .append(RcDoc::text(" && "))
                    .append(rhs)
                    .append(right)
            }
            BinOp::Or(ref lhs, ref rhs) => {
                let lhs = lhs.to_doc(op_prec);
                let rhs = rhs.to_doc(op_prec);
                left.append(lhs)
                    .append(RcDoc::text(" || "))
                    .append(rhs)
                    .append(right)
            }
            BinOp::Pow(ref lhs, ref rhs) => lhs
                .to_doc(precedence)
                .append(RcDoc::text(".powf(").append(rhs.to_doc(precedence)))
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
            FunctionName::Ceil => RcDoc::text("ceiling"),
            FunctionName::Trunc => RcDoc::text("floor"),
            FunctionName::Fract => RcDoc::text("fraction"),
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
            FunctionName::Normalize => RcDoc::text("normalize!"),
            FunctionName::RoundMin => RcDoc::text("round_min"),
            FunctionName::RoundMax => RcDoc::text("round_max"),
            FunctionName::SmoothAbs => RcDoc::text("smooth_abs"),
            FunctionName::PolySmoothAbs => RcDoc::text("poly_smooth_abs"),
            FunctionName::SmoothClamp => RcDoc::text("smooth_clamp"),
            FunctionName::PolySmoothClamp => RcDoc::text("poly_smooth_clamp"),
            FunctionName::ValueNoise => RcDoc::text("value_noise"),
            FunctionName::Torus => RcDoc::text("torus"),
            FunctionName::Box2 => RcDoc::text("box2"),
            FunctionName::Box3 => RcDoc::text("box3"),
            FunctionName::Rot0 => RcDoc::text("rot0"),
            FunctionName::Rot1 => RcDoc::text("rot0"),
            FunctionName::Rot => RcDoc::text("rot"),
            FunctionName::Triangle => RcDoc::text("triangle"),
            FunctionName::Corner => RcDoc::text("corner"),
            FunctionName::FakeSine => RcDoc::text("fake_sine"),
            FunctionName::Hash => RcDoc::text("hash"),
            FunctionName::AddMul => RcDoc::text("add_mul!"),
            FunctionName::Round => RcDoc::text("round"),
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::parser::parse;
    use crate::sdf::examples;

    #[test]
    fn show() {
        let examples = examples();
        let (mut input, _) = examples.get("ghost").unwrap();
        let ast = parse(&mut input);
        print!("{}", generate_code(&ast, 0.1, 0.2));
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
