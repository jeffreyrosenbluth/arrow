use crate::ast::*;
use crate::core::modulo;
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

pub fn eval(env: &mut Environment, ast: Statement, v: Vec3) -> f32 {
    match &ast {
        Statement::Assign { var, rhs } => {
            env.insert(var.clone(), eval_expr(env, rhs.clone()));
            0.0
        }
        Statement::AssignArray { vars, rhs } => {
            let value = eval_expr(env, rhs.clone());
            for var in vars {
                env.insert(var.clone(), value);
            }
            0.0
        }
        Statement::Sequence(stmts) => todo!(),
        Statement::ForNumeric { n, block } => todo!(),
        Statement::ForAlpha { a, block } => todo!(),
        Statement::Return(_) => todo!(),
    }
}

fn eval_expr(env: &Environment, ast: Box<Expr>) -> Value {
    use Value::*;
    match *ast {
        Expr::Scalar(value) => ScalarVal(value),
        Expr::BinaryOp(op) => eval_binop(env, op),
        Expr::UnaryOp => todo!(),
        Expr::Paren(expr) => eval_expr(env, expr),
        Expr::Function { name, args } => eval_function(env, name, args),
        Expr::Variable(name) => {
            let value = env.get(&name).expect("variable not found");
            value.clone()
        }
    }
}

fn eval_binop(env: &Environment, ast: BinOp) -> Value {
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

fn eval_function(env: &Environment, name: FunctionName, args: Vec<Expr>) -> Value {
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
                (ScalarVal(arg0), ScalarVal(arg1)) => ScalarVal(modulo(arg0, arg1)),
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
                    let t = (arg0 - arg1) / (arg2 - arg1);
                    ScalarVal(t * t * (3.0 - 2.0 * t))
                }
                _ => panic!("smoothstep expects scalar values"),
            }
        }
        Length => {
            let arg0 = eval_expr(env, Box::new(args[0].clone()));
            let arg1 = eval_expr(env, Box::new(args[1].clone()));
            let arg2 = eval_expr(env, Box::new(args[2].clone()));
            match (arg0, arg1, arg2) {
                (ScalarVal(arg0), ScalarVal(arg1), ScalarVal(arg2)) => {
                    ScalarVal(Vec3::new(arg0, arg1, arg2).length())
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
        Union => todo!(),
        Intersect => todo!(),
        AddMul => todo!(),
        ValueNoise => todo!(),
        Torus => todo!(),
        Box3 => todo!(),
        Floors => todo!(),
        Rot0 => todo!(),
        Rot1 => todo!(),
        Triangle => todo!(),
    }
}
