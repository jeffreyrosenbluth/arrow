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
                env.insert("$$".to_string(), ScalarVal(((i + 1) % n) as f32));
                env.insert("$$$".to_string(), ScalarVal(((i + 2) % n) as f32));
                eval(env, block, v);
            }
        }
        Statement::ForAlpha { a, block } => {
            let cs = a.chars().collect::<Vec<_>>();
            let n = cs.len();
            for i in 0..n {
                env.insert(
                    "$".to_string(),
                    env.get(&cs[i].to_string()).unwrap().clone(),
                );
                env.insert(
                    "$$".to_string(),
                    env.get(&cs[(i + 1) % n].to_string()).unwrap().clone(),
                );
                env.insert(
                    "$$$".to_string(),
                    env.get(&cs[(i + 2) % n].to_string()).unwrap().clone(),
                );
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
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => ScalarVal(x.sin()),
                _ => panic!("sin expects scalar values"),
            }
        }
        Cos => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => ScalarVal(x.cos()),
                _ => panic!("cos expects scalar values"),
            }
        }
        Tan => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            match x {
                ScalarVal(x) => ScalarVal(x.tan()),
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
        Mod => {
            let x = eval_expr(env, Box::new(args[0].clone()));
            let m = eval_expr(env, Box::new(args[1].clone()));
            match (x, m) {
                (ScalarVal(x), ScalarVal(m)) => ScalarVal(modulo(x, m) - 1.0 * m),
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
        Rot0 => todo!(),
        Rot1 => todo!(),
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
        SmoothClamp => {
            let arg0 = eval_expr(env, Box::new(args[0].clone()));
            let arg1 = eval_expr(env, Box::new(args[1].clone()));
            let arg2 = eval_expr(env, Box::new(args[2].clone()));
            let arg3 = eval_expr(env, Box::new(args[3].clone()));
            match (arg0, arg1, arg2, arg3) {
                (ScalarVal(arg0), ScalarVal(arg1), ScalarVal(arg2), ScalarVal(arg3)) => ScalarVal(
                    (smooth_abs(arg0 - arg2, arg1) - smooth_abs(arg0 - arg3, arg2) + arg2 + arg3)
                        / 2.0,
                ),
                _ => panic!("smoothclamp expects scalar values"),
            }
        }
    }
}

fn smooth_abs(x: f32, p: f32) -> f32 {
    (x * x + p).sqrt()
}
