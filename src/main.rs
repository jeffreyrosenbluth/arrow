use arrow::core::*;
use arrow::eval::make_sdf;
use arrow::march::render;
use arrow::sdf::examples;
use arrow::sdfs::*;

#[allow(unused_imports)]
use arrow::sdf::sd_plane;

const S: u32 = 1;
const WIDTH: u32 = 1 * 1024 / S;
const HEIGHT: u32 = 1 * 768 / S;
const AA: u32 = 1;

fn main() {
    use arrow::pratt::*;
    let background = 0.75;
    let examples = examples();
    let (mut input, pos) = *examples.get("thepath").unwrap();
    let ast = parse(&mut input);
    dbg!(&ast);
    // let sdf: Sdf = Box::new(move |p| make_sdf(&ast, 0.1, 0.2, p));
    let sdf: Sdf = Box::new(thepath);
    println!("sdf: {}", sdf(pos));
    let img_data = render(
        &sdf,
        // Camera position
        pos,
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
