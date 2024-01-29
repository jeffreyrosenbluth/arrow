use crate::ast::*;
use crate::core::{modulo, v3, I, ZERO3};
use crate::sdf::{sd_box, sd_torus};
use glam::{Vec2, Vec3};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    ScalarVal(f32),
    BoolVal(bool),
    Vec2Val(Vec2),
    Vec3Val(Vec3),
}

type Environment = HashMap<String, Value>;

pub fn eval(env: &mut Environment, ast: &Statement, v: Vec3) {
    use Value::*;
    env.insert("x".to_string(), ScalarVal(v.x));
    env.insert("y".to_string(), ScalarVal(v.y));
    env.insert("z".to_string(), ScalarVal(v.z));
    match &ast {
        Statement::Assign { var, rhs } => {
            let r = eval_expr(env, rhs.clone());
            env.insert(var.clone(), r);
        }
        Statement::AssignArray { vars, rhs } => {
            let value = eval_expr(env, rhs.clone());
            for var in vars {
                env.insert(var.clone(), value);
            }
        }
        Statement::Sequence(stmts) => {
            for s in stmts {
                eval(env, s, v);
            }
        }
        Statement::ForNumeric { n, block } => {
            for i in 0..*n {
                env.insert("$".to_string(), ScalarVal(i as f32));
                eval(env, block, v);
            }
        }
        Statement::ForAlpha { a, block } => {
            for c in a.chars() {
                env.insert("$".to_string(), env.get(&c.to_string()).unwrap().clone());
                eval(env, block, v);
            }
        }
        Statement::Return(expr) => {
            let _ = eval_expr(env, expr.clone());
        }
    }
}

fn eval_expr(env: &mut Environment, ast: Box<Expr>) -> Value {
    use Value::*;
    match *ast {
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
        Expr::UnaryOp => todo!(),
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
    }
}

fn eval_binop(env: &mut Environment, ast: BinOp) -> Value {
    use Value::*;
    match ast {
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
        BinOp::Eq(a, b) => {
            let a = eval_expr(env, a);
            let b = eval_expr(env, b);
            match (a, b) {
                (ScalarVal(a), ScalarVal(b)) => BoolVal(a == b),
                _ => panic!("== expects scalar values"),
            }
        }
    }
}

fn eval_function(env: &mut Environment, name: FunctionName, args: Vec<Expr>) -> Value {
    use FunctionName::*;
    use Value::*;
    match name {
        Sin => {
            let arg = eval_expr(env, Box::new(args[0].clone()));
            match arg {
                ScalarVal(arg) => ScalarVal(arg.sin()),
                _ => panic!("sin expects scalar values"),
            }
        }
        Cos => {
            let arg = eval_expr(env, Box::new(args[0].clone()));
            match arg {
                ScalarVal(arg) => ScalarVal(arg.cos()),
                _ => panic!("cos expects scalar values"),
            }
        }
        Tan => {
            let arg = eval_expr(env, Box::new(args[0].clone()));
            match arg {
                ScalarVal(arg) => ScalarVal(arg.tan()),
                _ => panic!("tan expects scalar values"),
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
        Exp => {
            let arg = eval_expr(env, Box::new(args[0].clone()));
            match arg {
                ScalarVal(arg) => ScalarVal(arg.exp()),
                _ => panic!("exp expects scalar values"),
            }
        }
        Exp2 => {
            let arg = eval_expr(env, Box::new(args[0].clone()));
            match arg {
                ScalarVal(arg) => ScalarVal(arg.exp2()),
                _ => panic!("exp2 expects scalar values"),
            }
        }
        Log => {
            let arg = eval_expr(env, Box::new(args[0].clone()));
            match arg {
                ScalarVal(arg) => ScalarVal(arg.ln()),
                _ => panic!("log expects scalar values"),
            }
        }
        Log2 => {
            let arg = eval_expr(env, Box::new(args[0].clone()));
            match arg {
                ScalarVal(arg) => ScalarVal(arg.log2()),
                _ => panic!("log2 expects scalar values"),
            }
        }
        Pow => {
            let arg0 = eval_expr(env, Box::new(args[0].clone()));
            let arg1 = eval_expr(env, Box::new(args[1].clone()));
            match (arg0, arg1) {
                (ScalarVal(arg0), ScalarVal(arg1)) => ScalarVal(arg0.powf(arg1)),
                _ => panic!("pow expects scalar values"),
            }
        }
        Sqrt => {
            let arg = eval_expr(env, Box::new(args[0].clone()));
            match arg {
                ScalarVal(arg) => ScalarVal(arg.sqrt()),
                _ => panic!("sqrt expects scalar values"),
            }
        }
        Abs => {
            let arg = eval_expr(env, Box::new(args[0].clone()));
            match arg {
                ScalarVal(arg) => ScalarVal(arg.abs()),
                _ => panic!("abs expects scalar values"),
            }
        }
        Sign => {
            let arg = eval_expr(env, Box::new(args[0].clone()));
            match arg {
                ScalarVal(arg) => ScalarVal(arg.signum()),
                _ => panic!("sign expects scalar values"),
            }
        }
        Floor => {
            let arg = eval_expr(env, Box::new(args[0].clone()));
            match arg {
                ScalarVal(arg) => ScalarVal(arg.floor()),
                _ => panic!("floor expects scalar values"),
            }
        }
        Ceil => {
            let arg = eval_expr(env, Box::new(args[0].clone()));
            match arg {
                ScalarVal(arg) => ScalarVal(arg.ceil()),
                _ => panic!("ceil expects scalar values"),
            }
        }
        Fract => {
            let arg = eval_expr(env, Box::new(args[0].clone()));
            match arg {
                ScalarVal(arg) => ScalarVal(arg.fract()),
                _ => panic!("fract expects scalar values"),
            }
        }
        Mod => {
            let arg0 = eval_expr(env, Box::new(args[0].clone()));
            let arg1 = eval_expr(env, Box::new(args[1].clone()));
            match (arg0, arg1) {
                (ScalarVal(arg0), ScalarVal(arg1)) => ScalarVal(modulo(arg0, arg1) - 1.0 * arg1),
                _ => panic!("mod expects scalar values"),
            }
        }
        Min => {
            let arg0 = eval_expr(env, Box::new(args[0].clone()));
            let arg1 = eval_expr(env, Box::new(args[1].clone()));
            match (arg0, arg1) {
                (ScalarVal(arg0), ScalarVal(arg1)) => ScalarVal(arg0.min(arg1)),
                _ => panic!("min expects scalar values"),
            }
        }
        Max => {
            let arg0 = eval_expr(env, Box::new(args[0].clone()));
            let arg1 = eval_expr(env, Box::new(args[1].clone()));
            match (arg0, arg1) {
                (ScalarVal(arg0), ScalarVal(arg1)) => ScalarVal(arg0.max(arg1)),
                _ => panic!("max expects scalar values"),
            }
        }
        Clamp => {
            let arg0 = eval_expr(env, Box::new(args[0].clone()));
            let arg1 = eval_expr(env, Box::new(args[1].clone()));
            let arg2 = eval_expr(env, Box::new(args[2].clone()));
            match (arg0, arg1, arg2) {
                (ScalarVal(arg0), ScalarVal(arg1), ScalarVal(arg2)) => {
                    ScalarVal(arg0.max(arg1).min(arg2))
                }
                _ => panic!("clamp expects scalar values"),
            }
        }
        Mix => {
            let arg0 = eval_expr(env, Box::new(args[0].clone()));
            let arg1 = eval_expr(env, Box::new(args[1].clone()));
            let arg2 = eval_expr(env, Box::new(args[2].clone()));
            match (arg0, arg1, arg2) {
                (ScalarVal(arg0), ScalarVal(arg1), ScalarVal(arg2)) => {
                    ScalarVal(arg0 * (1.0 - arg2) + arg1 * arg2)
                }
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
            let arg0 = eval_expr(env, Box::new(args[0].clone()));
            let arg1 = eval_expr(env, Box::new(args[1].clone()));
            let arg2 = if args.len() > 2 {
                eval_expr(env, Box::new(args[2].clone()))
            } else {
                ScalarVal(0.0)
            };
            match (arg0, arg1, arg2) {
                (ScalarVal(arg0), ScalarVal(arg1), ScalarVal(arg2)) => {
                    ScalarVal(v3(arg0, arg1, arg2).length())
                }
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
        Intersect => todo!(),
        AddMul => todo!(),
        ValueNoise => todo!(),
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
        Box3 => {
            let arg0 = eval_expr(env, Box::new(args[0].clone()));
            let arg1 = eval_expr(env, Box::new(args[1].clone()));
            let arg2 = eval_expr(env, Box::new(args[2].clone()));
            match (arg0, arg1, arg2) {
                (ScalarVal(arg0), ScalarVal(arg1), ScalarVal(arg2)) => {
                    let p = v3(arg0, arg1, arg2);
                    let sdf = sd_box(v3(4.0, 4.0, 0.0), v3(0.0, -20.0, 5.0), I);
                    ScalarVal(sdf(p))
                }
                _ => panic!("box3 expects scalar values"),
            }
        }
        Floors => todo!(),
        Rot0 => todo!(),
        Rot1 => todo!(),
        Triangle => todo!(),
    }
}
