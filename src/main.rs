use arrow::core::*;
use arrow::eval::*;
use arrow::march::render;
use arrow::sdf::examples;

#[allow(unused_imports)]
use arrow::sdf::sd_plane;

const S: u32 = 1;
const WIDTH: u32 = 1024 / S;
const HEIGHT: u32 = 768 / S;
const AA: u32 = 1;

fn main() {
    use arrow::pratt::*;
    let background = 0.75;
    let examples = examples();
    let (mut input, pos) = *examples.get("system").unwrap();
    let ast = parse(&mut input);
    dbg!(&ast);
    let sdf: Sdf = Box::new(move |p| make_sdf(&ast, 0.1, 0.2, p));
    println!("sdf: {}", sdf(v3(0.0, 0.0, -50.0)));
    // let plane = sd_plane(v3(0.0, 0.85, 0.3), 10.0, I);
    // let sdf = union(sdf, plane);
    let img_data = render(
        &sdf,
        // Camera position
        pos,
        // Look at
        ZERO3,
        &vec![
            Light::new(v3(0.0, 0.0, -50.0), 0.8),
            // Light::new(v3(0.0, 10.0, 40.0), 1.0),
        ],
        background,
        WIDTH,
        HEIGHT,
        AA, // Anti-aliasing
    );
    image::save_buffer("hatch.png", &img_data, WIDTH, HEIGHT, image::ColorType::L8).unwrap();
}
