use arrow::{codegen::generate_code, pratt::parse, sdf::examples};

fn main() {
    let examples = examples();
    let (mut input, _) = examples.get("pawns").unwrap();
    let ast = parse(&mut input);
    print!("{}", generate_code(&ast, 0.1, 0.2));
}
