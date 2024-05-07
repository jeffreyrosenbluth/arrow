use arrow::core::*;
// use arrow::eval::make_sdf;
use arrow::march::render;
// use arrow::sdf::examples;
use arrow::sdfs::*;
use rhai::{Dynamic, Engine, Scope};

#[allow(unused_imports)]
use arrow::sdf::sd_plane;

const S: u32 = 1;
const M: u32 = 1;
const WIDTH: u32 = M * 1024 / S;
const HEIGHT: u32 = M * 768 / S;
const AA: u32 = 3;

fn main() {
    let engine = Engine::new();
    let rhai_ast = dbg!(engine.compile_file("./src/functions.rhai".into())).unwrap();
    let mut scope = Scope::new();
    let r = engine.call_fn::<f32>(
        &mut scope,
        &rhai_ast,
        "union",
        vec![-2.25f32, 1f32, 1.2f32, 1.5f32, 2.5f32, 3.5f32],
    );
    _ = dbg!(r);

    // use arrow::pratt::*;
    let background = 0.75;
    // let examples = examples();
    // let (mut input, camera) = *examples.get("system").unwrap();
    // let ast = parse(&mut input);
    // dbg!(&ast);
    // let sdf: Sdf = Box::new(move |p| make_sdf(&ast, 0.2, 0.4, p));
    let Scene { sdf, camera } = scene("cross");
    let sdf: Sdf = Box::new(sdf);
    println!("sdf: {}", sdf(camera));
    let img_data = render(
        &sdf,
        // Camera position
        camera,
        // Look at
        ZERO3,
        &vec![
            Light::new(v3(0.0, 0.0, -50.0), 1.0),
            Light::new(v3(0.0, 10.0, 40.0), 1.0),
        ],
        background,
        WIDTH,
        HEIGHT,
        AA, // Anti-aliasing
    );
    image::save_buffer("hatch.png", &img_data, WIDTH, HEIGHT, image::ColorType::L8).unwrap();
}
