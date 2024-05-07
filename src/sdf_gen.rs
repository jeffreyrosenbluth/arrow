use arrow::{parser::parse, rhai_gen::generate_code, sdf::examples};

fn main() {
    let examples = examples();
    let (mut input, _) = examples.get("sponge").unwrap();
    let ast = parse(&mut input);
    print!("{}", generate_code(&ast, 0.1, 0.2));
}
