use arrow::{lua_gen::generate_code, parser::parse, sdf::examples};

fn main() {
    let examples = examples();
    let (mut input, _) = examples.get("asurf").unwrap();
    let ast = parse(&mut input);
    print!("{}", generate_code(&ast, 0.1, 0.2));
}
