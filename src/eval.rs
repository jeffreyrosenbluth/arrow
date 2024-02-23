use crate::ast::*;
use crate::core::{fbm, modulo, v3, I, ZERO3};
use crate::sdf::{sd_box, sd_torus};
use glam::{Mat2, Vec2, Vec3};
use std::collections::HashMap;
use std::f32::consts::TAU;

pub fn make_sdf(ast: &Statement, a0: f32, a1: f32, p: Vec3) -> f32 {
    let mut env = HashMap::new();
    env.insert("a0".to_string(), Value::ScalarVal(a0));
    env.insert("a1".to_string(), Value::ScalarVal(a1));
    eval(&mut env, &ast, p);
    let v = env.get("#").unwrap();
    match v {
        Value::ScalarVal(s) => *s,
        _ => panic!("sd is not a scalar"),
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Value {
    ScalarVal(f32),
    BoolVal(bool),
    Vec2Val(Vec2),
    Vec3Val(Vec3),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::ScalarVal(a), Value::ScalarVal(b)) => a == b,
            (Value::BoolVal(a), Value::BoolVal(b)) => a == b,
            (Value::Vec2Val(a), Value::Vec2Val(b)) => a == b,
            (Value::Vec3Val(a), Value::Vec3Val(b)) => a == b,
            _ => panic!("eq expects scalar values"),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::ScalarVal(a), Value::ScalarVal(b)) => a.partial_cmp(b),
            (Value::BoolVal(a), Value::BoolVal(b)) => a.partial_cmp(b),
            _ => panic!("partial_cmp expects scalar values"),
        }
    }
}

pub type Environment = HashMap<String, Value>;

pub fn eval(env: &mut Environment, ast: &Statement, v: Vec3) {
    use Value::*;
    if !env.contains_key("x") {
        env.insert("x".to_string(), ScalarVal(v.x));
    }
    if !env.contains_key("y") {
        env.insert("y".to_string(), ScalarVal(v.y));
    }
    if !env.contains_key("z") {
        env.insert("z".to_string(), ScalarVal(v.z));
    }
    match &ast {
        Statement::Assign { var, rhs } => {
            let r = eval_expr(env, rhs.clone());
            env.insert(var.clone(), r);
        }
        Statement::AssignArray { vars, rhs } => {
            let value = eval_expr(env, rhs.clone());
            match value {
                Vec2Val(v) => {
                    env.insert(vars[0].clone(), ScalarVal(v.x));
                    env.insert(vars[1].clone(), ScalarVal(v.y));
                }
                ScalarVal(_) => panic!("assign array expects vector values"),
                BoolVal(_) => panic!("assign array expects vector values"),
                Vec3Val(v) => {
                    env.insert(vars[0].clone(), ScalarVal(v.x));
                    env.insert(vars[1].clone(), ScalarVal(v.y));
                    env.insert(vars[2].clone(), ScalarVal(v.z));
                }
            }
        }
        Statement::Sequence(stmts) => {
            for s in stmts {
                eval(env, s, v);
            }
        }
        Statement::Return(expr) => {
            let _ = eval_expr(env, expr.clone());
        }
        Statement::Empty => {}
    }
}

fn eval_expr(env: &mut Environment, ast: Box<Expr>) -> Value {
    use Value::*;
    match *ast {
        Expr::Negate(expr) => {
            let r = eval_expr(env, expr);
            match r {
                ScalarVal(r) => ScalarVal(-r),
                _ => panic!("negate expects scalar values"),
            }
        }
        Expr::Scalar(value) => {
            let r = ScalarVal(value);
            env.insert("#".to_string(), r);
            r
        }
        Expr::BinaryOp(op) => {
            let r = eval_binop(env, op);
            env.insert("#".to_string(), r);
            r
        }
        Expr::Paren(expr) => {
            let r = eval_expr(env, expr);
            env.insert("#".to_string(), r);
            r
        }
        Expr::Function { name, args } => {
            let r = eval_function(env, name, args);
            env.insert("#".to_string(), r);
            r
        }
        Expr::Variable(name) => {
            let value = env.get(&name).expect("variable not found").clone();
            env.insert("#".to_string(), value);
            value
        }
        Expr::TernaryOp(cond, if_true, if_false) => {
            let cond = eval_expr(env, cond);
            let value = match cond {
                BoolVal(true) => eval_expr(env, if_true),
                BoolVal(false) => eval_expr(env, if_false),
                _ => panic!("ternary expects boolean values"),
            };
            env.insert("#".to_string(), value);
            value
        }
    }
}

fn eval_binop(env: &mut Environment, ast: BinOp) -> Value {
    use Value::*;
    match ast {
        BinOp::Eq(a, b) => {
            let a = eval_expr(env, a);
            let b = eval_expr(env, b);
            match (a, b) {
                (ScalarVal(a), ScalarVal(b)) => BoolVal(a == b),
                _ => panic!("== expects scalar values"),
            }
        }
        BinOp::Greater(a, b) => {
            let a = eval_expr(env, a);
            let b = eval_expr(env, b);
            match (a, b) {
                (ScalarVal(a), ScalarVal(b)) => BoolVal(a > b),
                _ => panic!("> expects scalar values"),
            }
        }
        BinOp::GreaterEq(a, b) => {
            let a = eval_expr(env, a);
            let b = eval_expr(env, b);
            match (a, b) {
                (ScalarVal(a), ScalarVal(b)) => BoolVal(a >= b),
                _ => panic!(">= expects scalar values"),
            }
        }
        BinOp::Less(a, b) => {
            let a = eval_expr(env, a);
            let b = eval_expr(env, b);
            match (a, b) {
                (ScalarVal(a), ScalarVal(b)) => BoolVal(a < b),
                _ => panic!("< expects scalar values"),
            }
        }
        BinOp::LessEq(a, b) => {
            let a = eval_expr(env, a);
            let b = eval_expr(env, b);
            match (a, b) {
                (ScalarVal(a), ScalarVal(b)) => BoolVal(a <= b),
                _ => panic!("<= expects scalar values"),
            }
        }
        BinOp::Add(a, b) => {
            let a = eval_expr(env, a);
            let b = eval_expr(env, b);
            match (a, b) {
                (ScalarVal(a), ScalarVal(b)) => ScalarVal(a + b),
                _ => panic!("+ expects scalar values"),
            }
        }
        BinOp::Sub(a, b) => {
            let a = eval_expr(env, a);
            let b = eval_expr(env, b);
            match (a, b) {
                (ScalarVal(a), ScalarVal(b)) => ScalarVal(a - b),
                _ => panic!("- expects scalar values"),
            }
        }
        BinOp::Mul(a, b) => {
            let a = eval_expr(env, a);
            let b = eval_expr(env, b);
            match (a, b) {
                (ScalarVal(a), ScalarVal(b)) => ScalarVal(a * b),
                _ => panic!("* expects scalar values"),
            }
        }
        BinOp::Div(a, b) => {
            let a = eval_expr(env, a);
            let b = eval_expr(env, b);
            match (a, b) {
                (ScalarVal(a), ScalarVal(b)) => ScalarVal(a / b),
                _ => panic!("/ expects scalar values"),
            }
        }
    }
}

fn eval_function(env: &mut Environment, name: FunctionName, args: Vec<Expr>) -> Value {
    use FunctionName::*;
    use Value::*;
    match name {
        Sin => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => ScalarVal(x.sin()),
                _ => panic!("sin expects scalar values"),
            }
        }
        Asin => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => ScalarVal(x.asin()),
                _ => panic!("asin expects scalar values"),
            }
        }
        Sinh => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => ScalarVal(x.sinh()),
                _ => panic!("sinh expects scalar values"),
            }
        }
        Asinh => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => ScalarVal(x.asinh()),
                _ => panic!("asinh expects scalar values"),
            }
        }
        FakeSine => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => {
                    ScalarVal(((x - x.floor() - 0.5) * 2.0).abs() * x * (6.0 - 4.0 * x) - 1.0)
                }
                _ => panic!("fakesine expects scalar values"),
            }
        }
        Cos => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => ScalarVal(x.cos()),
                _ => panic!("cos expects scalar values"),
            }
        }
        Acos => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => ScalarVal(x.acos()),
                _ => panic!("acos expects scalar values"),
            }
        }
        Cosh => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => ScalarVal(x.cosh()),
                _ => panic!("cosh expects scalar values"),
            }
        }
        Acosh => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => ScalarVal(x.acosh()),
                _ => panic!("acosh expects scalar values"),
            }
        }
        Tan => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => ScalarVal(x.tan()),
                _ => panic!("tan expects scalar values"),
            }
        }
        Atan => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => ScalarVal(x.atan()),
                _ => panic!("atan expects scalar values"),
            }
        }
        Atan2 => {
            let arg0 = eval_expr(env, Box::new(args[0].clone()));
            let arg1 = eval_expr(env, Box::new(args[1].clone()));
            match (arg0, arg1) {
                (ScalarVal(arg0), ScalarVal(arg1)) => ScalarVal(arg0.atan2(arg1)),
                _ => panic!("atan2 expects scalar values"),
            }
        }
        Tanh => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => ScalarVal(x.tanh()),
                _ => panic!("tanh expects scalar values"),
            }
        }
        Atanh => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => ScalarVal(x.atanh()),
                _ => panic!("atanh expects scalar values"),
            }
        }
        Exp => {
            let arg = eval_expr(env, Box::new(args[0].clone()));
            match arg {
                ScalarVal(x) => ScalarVal(x.exp()),
                _ => panic!("exp expects scalar values"),
            }
        }
        Exp2 => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => ScalarVal(x.exp2()),
                _ => panic!("exp2 expects scalar values"),
            }
        }
        Log => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => ScalarVal(x.ln()),
                _ => panic!("log expects scalar values"),
            }
        }
        Log2 => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => ScalarVal(x.log2()),
                _ => panic!("log2 expects scalar values"),
            }
        }
        Pow => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            let p = eval_expr(env, Box::new(args[1].clone()));
            match (x, p) {
                (ScalarVal(x), ScalarVal(p)) => ScalarVal(x.powf(p)),
                _ => panic!("pow expects scalar values"),
            }
        }
        Sqrt => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(p) => ScalarVal(p.sqrt()),
                _ => panic!("sqrt expects scalar values"),
            }
        }
        Abs => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => ScalarVal(x.abs()),
                _ => panic!("abs expects scalar values"),
            }
        }
        Sign => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => ScalarVal(x.signum()),
                _ => panic!("sign expects scalar values"),
            }
        }
        Floor => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => ScalarVal(x.floor()),
                _ => panic!("floor expects scalar values"),
            }
        }
        Trunc => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => ScalarVal(x.trunc()),
                _ => panic!("trunc expects scalar values"),
            }
        }
        Ceil => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => ScalarVal(x.ceil()),
                _ => panic!("ceil expects scalar values"),
            }
        }
        Fract => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => ScalarVal(x.fract()),
                _ => panic!("fract expects scalar values"),
            }
        }
        Round => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => ScalarVal(x.round()),
                _ => panic!("round expects scalar values"),
            }
        }
        Mod => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            let m = eval_expr(env, Box::new(args[1].clone()));
            match (x, m) {
                (ScalarVal(x), ScalarVal(m)) => ScalarVal(modulo(x, m) - m),
                _ => panic!("mod expects scalar values"),
            }
        }
        Min => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            let y = eval_expr(env, Box::new(args[1].clone()));
            match (x, y) {
                (ScalarVal(x), ScalarVal(y)) => ScalarVal(x.min(y)),
                _ => panic!("min expects scalar values"),
            }
        }
        Max => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            let y = eval_expr(env, Box::new(args[1].clone()));
            match (x, y) {
                (ScalarVal(x), ScalarVal(y)) => ScalarVal(x.max(y)),
                _ => panic!("max expects scalar values"),
            }
        }
        Clamp => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            let a = eval_expr(env, Box::new(args[1].clone()));
            let b = eval_expr(env, Box::new(args[2].clone()));
            match (x, a, b) {
                (ScalarVal(x), ScalarVal(a), ScalarVal(b)) => ScalarVal(x.max(a).min(b)),
                _ => panic!("clamp expects scalar values"),
            }
        }
        Mix => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            let y = eval_expr(env, Box::new(args[1].clone()));
            let t = eval_expr(env, Box::new(args[2].clone()));
            match (x, y, t) {
                (ScalarVal(x), ScalarVal(y), ScalarVal(t)) => ScalarVal(x * (1.0 - t) + y * t),
                _ => panic!("mix expects scalar values"),
            }
        }
        Smoothstep => {
            let arg0 = eval_expr(env, Box::new(args[0].clone()));
            let arg1 = eval_expr(env, Box::new(args[1].clone()));
            let arg2 = eval_expr(env, Box::new(args[2].clone()));
            match (arg0, arg1, arg2) {
                (ScalarVal(arg0), ScalarVal(arg1), ScalarVal(arg2)) => {
                    let t = ((arg2 - arg0) / (arg1 - arg0)).clamp(0.0, 1.0);
                    ScalarVal(t * t * (3.0 - 2.0 * t))
                }
                _ => panic!("smoothstep expects scalar values"),
            }
        }
        Length => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            let y = eval_expr(env, Box::new(args[1].clone()));
            let z = if args.len() > 2 {
                eval_expr(env, Box::new(args[2].clone()))
            } else {
                ScalarVal(0.0)
            };
            match (x, y, z) {
                (ScalarVal(x), ScalarVal(y), ScalarVal(z)) => ScalarVal(v3(x, y, z).length()),
                _ => panic!("length expects scalar values"),
            }
        }
        Distance => {
            let arg0 = eval_expr(env, Box::new(args[0].clone()));
            let arg1 = eval_expr(env, Box::new(args[1].clone()));
            let arg2 = eval_expr(env, Box::new(args[2].clone()));
            let arg3 = eval_expr(env, Box::new(args[3].clone()));
            let arg4 = eval_expr(env, Box::new(args[4].clone()));
            let arg5 = eval_expr(env, Box::new(args[5].clone()));
            match (arg0, arg1, arg2, arg3, arg4, arg5) {
                (
                    ScalarVal(arg0),
                    ScalarVal(arg1),
                    ScalarVal(arg2),
                    ScalarVal(arg3),
                    ScalarVal(arg4),
                    ScalarVal(arg5),
                ) => ScalarVal(Vec3::new(arg0, arg1, arg2).distance(Vec3::new(arg3, arg4, arg5))),
                _ => panic!("distance expects scalar values"),
            }
        }
        Dot => {
            let arg0 = eval_expr(env, Box::new(args[0].clone()));
            let arg1 = eval_expr(env, Box::new(args[1].clone()));
            let arg2 = eval_expr(env, Box::new(args[2].clone()));
            let arg3 = eval_expr(env, Box::new(args[3].clone()));
            let arg4 = eval_expr(env, Box::new(args[4].clone()));
            let arg5 = eval_expr(env, Box::new(args[5].clone()));
            match (arg0, arg1, arg2, arg3, arg4, arg5) {
                (
                    ScalarVal(arg0),
                    ScalarVal(arg1),
                    ScalarVal(arg2),
                    ScalarVal(arg3),
                    ScalarVal(arg4),
                    ScalarVal(arg5),
                ) => ScalarVal(Vec3::new(arg0, arg1, arg2).dot(Vec3::new(arg3, arg4, arg5))),
                _ => panic!("dot expects scalar values"),
            }
        }
        Cross => {
            let arg0 = eval_expr(env, Box::new(args[0].clone()));
            let arg1 = eval_expr(env, Box::new(args[1].clone()));
            let arg2 = eval_expr(env, Box::new(args[2].clone()));
            let arg3 = eval_expr(env, Box::new(args[3].clone()));
            let arg4 = eval_expr(env, Box::new(args[4].clone()));
            let arg5 = eval_expr(env, Box::new(args[5].clone()));
            match (arg0, arg1, arg2, arg3, arg4, arg5) {
                (
                    ScalarVal(arg0),
                    ScalarVal(arg1),
                    ScalarVal(arg2),
                    ScalarVal(arg3),
                    ScalarVal(arg4),
                    ScalarVal(arg5),
                ) => Vec3Val(Vec3::new(arg0, arg1, arg2).cross(Vec3::new(arg3, arg4, arg5))),
                _ => panic!("cross expects scalar values"),
            }
        }
        Normalize => {
            let arg0 = eval_expr(env, Box::new(args[0].clone()));
            let arg1 = eval_expr(env, Box::new(args[1].clone()));
            let arg2 = eval_expr(env, Box::new(args[2].clone()));
            match (arg0, arg1, arg2) {
                (ScalarVal(arg0), ScalarVal(arg1), ScalarVal(arg2)) => {
                    Vec3Val(Vec3::new(arg0, arg1, arg2).normalize())
                }
                _ => panic!("normalize expects scalar values"),
            }
        }
        Union => {
            let ds: Vec<Value> = args
                .into_iter()
                .map(|arg| eval_expr(env, Box::new(arg)))
                .collect();
            let min = ds.iter().min_by(|a, b| {
                let a = if let ScalarVal(a) = a {
                    a
                } else {
                    panic!("union expects scalar values")
                };
                let b = if let ScalarVal(b) = b {
                    b
                } else {
                    panic!("union expects scalar values")
                };
                a.partial_cmp(b).unwrap()
            });
            min.unwrap().clone()
        }
        Intersect => {
            let ds: Vec<Value> = args
                .into_iter()
                .map(|arg| eval_expr(env, Box::new(arg)))
                .collect();
            let max = ds.iter().max_by(|a, b| {
                let a = if let ScalarVal(a) = a {
                    a
                } else {
                    panic!("union expects scalar values")
                };
                let b = if let ScalarVal(b) = b {
                    b
                } else {
                    panic!("union expects scalar values")
                };
                a.partial_cmp(b).unwrap()
            });
            max.unwrap().clone()
        }
        AddMul => {
            let (x, y, z, a, b, c, t) = if args.len() == 4 {
                (
                    eval_expr(env, Box::new(args[0].clone())),
                    eval_expr(env, Box::new(args[1].clone())),
                    ScalarVal(0.0),
                    eval_expr(env, Box::new(args[2].clone())),
                    eval_expr(env, Box::new(args[3].clone())),
                    ScalarVal(0.0),
                    ScalarVal(1.0),
                )
            } else if args.len() == 5 {
                (
                    eval_expr(env, Box::new(args[0].clone())),
                    eval_expr(env, Box::new(args[1].clone())),
                    ScalarVal(0.0),
                    eval_expr(env, Box::new(args[2].clone())),
                    eval_expr(env, Box::new(args[3].clone())),
                    ScalarVal(0.0),
                    eval_expr(env, Box::new(args[4].clone())),
                )
            } else if args.len() == 6 {
                (
                    eval_expr(env, Box::new(args[0].clone())),
                    eval_expr(env, Box::new(args[1].clone())),
                    eval_expr(env, Box::new(args[2].clone())),
                    eval_expr(env, Box::new(args[3].clone())),
                    eval_expr(env, Box::new(args[4].clone())),
                    eval_expr(env, Box::new(args[5].clone())),
                    ScalarVal(1.0),
                )
            } else {
                (
                    eval_expr(env, Box::new(args[0].clone())),
                    eval_expr(env, Box::new(args[1].clone())),
                    eval_expr(env, Box::new(args[2].clone())),
                    eval_expr(env, Box::new(args[3].clone())),
                    eval_expr(env, Box::new(args[4].clone())),
                    eval_expr(env, Box::new(args[5].clone())),
                    eval_expr(env, Box::new(args[6].clone())),
                )
            };
            match (x, y, z, a, b, c, t) {
                (
                    ScalarVal(x),
                    ScalarVal(y),
                    ScalarVal(z),
                    ScalarVal(a),
                    ScalarVal(b),
                    ScalarVal(c),
                    ScalarVal(t),
                ) => Vec3Val(v3(x + a * t, y + b * t, z + c * t)),
                _ => panic!("addmul expects scalar values"),
            }
        }
        Torus => {
            let arg0 = eval_expr(env, Box::new(args[0].clone()));
            let arg1 = eval_expr(env, Box::new(args[1].clone()));
            let arg2 = eval_expr(env, Box::new(args[2].clone()));
            let arg3 = eval_expr(env, Box::new(args[3].clone()));
            let arg4 = eval_expr(env, Box::new(args[4].clone()));

            match (arg0, arg1, arg2, arg3, arg4) {
                (
                    ScalarVal(arg0),
                    ScalarVal(arg1),
                    ScalarVal(arg2),
                    ScalarVal(arg3),
                    ScalarVal(arg4),
                ) => {
                    let p = v3(arg0, arg1, arg2);
                    let sdf = sd_torus(arg3, arg4, ZERO3, I);
                    ScalarVal(sdf(p))
                }
                _ => panic!("torus expects scalar values"),
            }
        }
        // bx2=(x,y,a,b=a)=>(x=abs(x)-a,y=abs(y)-b,x>0&&y>0?L(x,y):x>y?x:y)
        Box2 => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            let y = eval_expr(env, Box::new(args[1].clone()));
            let a = eval_expr(env, Box::new(args[2].clone()));
            let b = if args.len() > 3 {
                eval_expr(env, Box::new(args[3].clone()))
            } else {
                a
            };
            match (x, y, a, b) {
                (ScalarVal(x), ScalarVal(y), ScalarVal(a), ScalarVal(b)) => {
                    let x = x.abs() - a;
                    let y = y.abs() - b;
                    if x > 0.0 && y > 0.0 {
                        ScalarVal(v3(x, y, 0.0).length())
                    } else {
                        ScalarVal(x.max(y))
                    }
                }
                _ => panic!("box2 expects scalar values"),
            }
        }
        Box3 => {
            let arg0 = eval_expr(env, Box::new(args[0].clone()));
            let arg1 = eval_expr(env, Box::new(args[1].clone()));
            let arg2 = eval_expr(env, Box::new(args[2].clone()));
            let arg3 = eval_expr(env, Box::new(args[3].clone()));
            let arg4 = if args.len() > 4 {
                eval_expr(env, Box::new(args[4].clone()))
            } else {
                arg3
            };
            let arg5 = if args.len() > 5 {
                eval_expr(env, Box::new(args[5].clone()))
            } else {
                arg3
            };
            match (arg0, arg1, arg2, arg3, arg4, arg5) {
                (
                    ScalarVal(arg0),
                    ScalarVal(arg1),
                    ScalarVal(arg2),
                    ScalarVal(arg3),
                    ScalarVal(arg4),
                    ScalarVal(arg5),
                ) => {
                    let p = v3(arg0, arg1, arg2);
                    let b = v3(arg3, arg4, arg5);
                    let sdf = sd_box(b, ZERO3, I);
                    ScalarVal(sdf(p))
                }
                _ => panic!("box3 expects scalar values"),
            }
        }
        Floors => todo!(),
        Rot0 => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            let y = eval_expr(env, Box::new(args[1].clone()));
            let a = env.get("a0").unwrap();
            match (x, y, a) {
                (ScalarVal(x), ScalarVal(y), ScalarVal(a)) => {
                    let v = Vec2::new(x, y);
                    let a = a * TAU;
                    let m = Mat2::from_angle(a);
                    Vec2Val(m * v)
                }
                _ => panic!("rot0 expects scalar values"),
            }
        }
        Rot1 => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            let y = eval_expr(env, Box::new(args[1].clone()));
            let a = env.get("a1").unwrap();
            match (x, y, a) {
                (ScalarVal(x), ScalarVal(y), ScalarVal(a)) => {
                    let v = Vec2::new(x, y);
                    let a = a * TAU;
                    let m = Mat2::from_angle(a);
                    Vec2Val(m * v)
                }
                _ => panic!("rot1 expects scalar values"),
            }
        }
        Triangle => {
            let arg0 = eval_expr(env, Box::new(args[0].clone()));
            match arg0 {
                ScalarVal(arg0) => ScalarVal((arg0 - (arg0 / 4.0).floor() * 4.0 - 2.0).abs() - 1.0),
                _ => panic!("triangle expects scalar values"),
            }
        }
        Corner => {
            let arg0 = eval_expr(env, Box::new(args[0].clone()));
            let arg1 = eval_expr(env, Box::new(args[1].clone()));
            match (arg0, arg1) {
                (ScalarVal(arg0), ScalarVal(arg1)) => {
                    if arg0 > 0.0 && arg1 > 0.0 {
                        ScalarVal(v3(arg0, arg1, 0.0).length())
                    } else {
                        ScalarVal(arg0.max(arg1))
                    }
                }
                _ => panic!("corner expects scalar values"),
            }
        }
        SmoothAbs => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            let p = if args.len() > 1 {
                eval_expr(env, Box::new(args[1].clone()))
            } else {
                ScalarVal(0.5)
            };
            match (x, p) {
                (ScalarVal(x), ScalarVal(p)) => ScalarVal(smooth_abs(x, p)),
                _ => panic!("smoothabs expects scalar values"),
            }
        }
        PolySmoothAbs => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            let m = if args.len() > 1 {
                eval_expr(env, Box::new(args[1].clone()))
            } else {
                ScalarVal(0.5)
            };
            match (x, m) {
                (ScalarVal(x), ScalarVal(p)) => ScalarVal(poly_smooth_abs(x, p)),
                _ => panic!("smoothabs expects scalar values"),
            }
        }
        SmoothClamp => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            let p = eval_expr(env, Box::new(args[1].clone()));
            let a = eval_expr(env, Box::new(args[2].clone()));
            let b = eval_expr(env, Box::new(args[3].clone()));
            match (x, p, a, b) {
                (ScalarVal(x), ScalarVal(p), ScalarVal(a), ScalarVal(b)) => {
                    ScalarVal((smooth_abs(x - a, p) - smooth_abs(x - b, p) + a + b) / 2.0)
                }
                _ => panic!("smoothclamp expects scalar values"),
            }
        }
        // qcl=(x,p,a,b)=>(qB(x-a,p)-qB(x-b,p)+b+a)/2
        PolySmoothClamp => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            let p = eval_expr(env, Box::new(args[1].clone()));
            let a = eval_expr(env, Box::new(args[2].clone()));
            let b = eval_expr(env, Box::new(args[3].clone()));
            match (x, p, a, b) {
                (ScalarVal(x), ScalarVal(p), ScalarVal(a), ScalarVal(b)) => {
                    ScalarVal((poly_smooth_abs(x - a, p) - poly_smooth_abs(x - b, p) + a + b) / 2.0)
                }
                _ => panic!("smoothclamp expects scalar values"),
            }
        }
        RoundMax => {
            let a = eval_expr(env, Box::new(args[0].clone()));
            let b = eval_expr(env, Box::new(args[1].clone()));
            let r = eval_expr(env, Box::new(args[2].clone()));
            match (a, b, r) {
                (ScalarVal(a), ScalarVal(b), ScalarVal(r)) => ScalarVal(if -a < r && -b < r {
                    Vec2::new(r + a, r + b).length() - r
                } else {
                    a.max(b)
                }),
                _ => panic!("roundmax expects scalar values"),
            }
        }
        // RoundMin => {
        //     let a = eval_expr(env, Box::new(args[0].clone()));
        //     let b = eval_expr(env, Box::new(args[1].clone()));
        //     let r = eval_expr(env, Box::new(args[2].clone()));
        //     match (a, b, r) {
        //         (ScalarVal(a), ScalarVal(b), ScalarVal(r)) => ScalarVal(smooth_min(a, b, r)),
        //         _ => panic!("roundmax expects scalar values"),
        //     }
        // }
        RoundMin => {
            let mut ds: Vec<f32> = args
                .into_iter()
                .map(|arg| {
                    if let ScalarVal(v) = eval_expr(env, Box::new(arg)) {
                        v
                    } else {
                        f32::MAX
                    }
                })
                .collect();
            let r = ds.pop().unwrap();
            let m = ds
                .into_iter()
                .reduce(|acc, e| smooth_min(acc, e, r))
                .unwrap();
            ScalarVal(m)
        }
        ValueNoise => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            let y = eval_expr(env, Box::new(args[1].clone()));
            let z = eval_expr(env, Box::new(args[2].clone()));
            let scale = eval_expr(env, Box::new(args[3].clone()));
            let offset = eval_expr(env, Box::new(args[4].clone()));
            let octaves = if args.len() > 5 {
                eval_expr(env, Box::new(args[5].clone()))
            } else {
                ScalarVal(1.0)
            };
            match (x, y, z, scale, offset, octaves) {
                (
                    ScalarVal(x),
                    ScalarVal(y),
                    ScalarVal(z),
                    ScalarVal(scale),
                    ScalarVal(offset),
                    ScalarVal(octaves),
                ) => ScalarVal(fbm(x, y, z, scale, offset, octaves as u32)),
                _ => panic!("noise expects scalar values"),
            }
        }
    }
}

fn smooth_abs(x: f32, p: f32) -> f32 {
    (x * x + p).sqrt()
}

fn smooth_min(a: f32, b: f32, r: f32) -> f32 {
    if a < r && b < r {
        r - Vec2::new(r - a, r - b).length()
    } else {
        a.min(b)
    }
}

fn poly_smooth_abs(x: f32, m: f32) -> f32 {
    if x.abs() > m {
        x
    } else {
        (2.0 - x / m) * x * x / m
    }
}
