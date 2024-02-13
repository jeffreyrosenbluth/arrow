use arrow::ast::Statement;
use arrow::core::v3;
use arrow::eval::{eval, Value};
use arrow::parser::program;
use glam::Vec3;
use std::collections::HashMap;

pub fn build_sdf(ast: &Statement, a0: f32, a1: f32, p: Vec3) -> f32 {
    let mut env = HashMap::new();
    env.insert("a0".to_string(), Value::ScalarVal(a0));
    env.insert("a1".to_string(), Value::ScalarVal(a1));
    eval(&mut env, &ast, p);
    println!("Env: {:?}", env);
    let v = env.get("#").unwrap();
    match v {
        Value::ScalarVal(s) => *s,
        _ => panic!("sd is not a scalar"),
    }
}

#[test]
fn sdf() {
    // let mut input = "[x,z]=r0(x-20,z), bx3(x,mod(y,1)-.5,mod(z,1)-.5,.45)";
    let mut rot_cube = "[a,b]=r0(x,y-9); bx3(a,b,z,4)-.5";
    let ast = program(&mut rot_cube).unwrap();
    let sdf = |p| build_sdf(&ast, 0.125, 0.2, p);
    println!("Sdf {:?}", sdf(v3(0.0, 0.0, -100.0)));
    println!("Ast: {:?}", &ast);
}
