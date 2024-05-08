use arrow::core::*;
// use arrow::eval::make_sdf;
use arrow::march::render;
use arrow::parser::parse;
use arrow::rhai_gen::{base_engine, generate_code};
use arrow::sdf::examples;
use arrow::sdfs::*;
use glam::Vec3;
use rhai::{Dynamic, Engine, Scope};

#[allow(unused_imports)]
use arrow::sdf::sd_plane;

const S: u32 = 1;
const M: u32 = 1;
const WIDTH: u32 = M * 1024 / S;
const HEIGHT: u32 = M * 768 / S;
const AA: u32 = 1;

fn main() {
    let mut engine = base_engine();
    let (mut input, camera) = examples().get("cross").unwrap();
    let ast = parse(&mut input);
    let script = generate_code(&ast, 0.1, 0.2);
    let rhai_ast = engine.compile(&script).unwrap();
    engine.set_optimization_level(rhai::OptimizationLevel::Full);
    let sdf = move |p: Vec3| -> f32 {
        let mut scope = Scope::new();
        engine
            .call_fn::<f32>(
                &mut scope,
                &rhai_ast,
                "signed_distance_function",
                (p.x, p.y, p.z),
            )
            .unwrap()
    };

    let background = 0.75;
    // let examples = examples();
    // let (mut input, camera) = *examples.get("system").unwrap();
    // let ast = parse(&mut input);
    // dbg!(&ast);
    // let sdf: Sdf = Box::new(move |p| make_sdf(&ast, 0.2, 0.4, p));
    let Scene { sdf: _, camera } = scene("cross");
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
