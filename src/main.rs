use arrow::core::*;
use arrow::eval::*;
use arrow::march::render;
use arrow::sdf::examples;

#[allow(unused_imports)]
use arrow::sdf::sd_plane;
use glam::{Vec2, Vec3};

const S: u32 = 1;
const WIDTH: u32 = 1024 / S;
const HEIGHT: u32 = 768 / S;
const AA: u32 = 2;

fn main() {
    use arrow::pratt::*;
    let background = 0.75;
    let examples = examples();
    let (mut input, pos) = *examples.get("ghost").unwrap();
    let ast = parse(&mut input);
    dbg!(&ast);
    // let sdf: Sdf = Box::new(move |p| make_sdf(&ast, 0.2, 0.4, p));
    let sdf: Sdf = Box::new(signed_distance_fucntion);
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

fn signed_distance_fucntion(p: Vec3) -> f32 {
    use arrow::functions::*;
    let Vec3 { x, y, z } = p;
    let (_, a1) = (0.1, 0.2);
    let y = y - 9.8f32;
    let [x, z] = rot0(x, z);
    let a = x;
    let b = y;
    let c = abs(z) - 0.3f32;
    let [a, b] = rot(a, b, cos(0.17f32), sin(0.17f32));
    let an = floor(0.5f32 + atan2(b, a) / a1) * a1;
    let [a, b] = rot(a, b, cos(an), sin(an));
    let d = intersect(vec![
        union(vec![
            box3(a - 7f32, b, c, 0.01f32, 2f32, 0.01f32) - 0.05f32,
            box3(b, a, c, 0.02f32, 7f32, 0.02f32) - 0.01f32,
            length(a - 7f32, b, 0.0) - 0.4f32,
            length(modulo(clamp(a, 0f32, 5f32), 1f32) - 0.5f32, b, 0.0) - 0.05f32,
        ]),
        abs(z) - 0.3f32,
    ]);
    let a = abs(x);
    let b = y;
    let an = 0.3f32;
    let [a, _] = rot(a, b, cos(an), sin(an));
    let d = union(vec![
        intersect(vec![
            union(vec![
                d,
                length(x, y, 0.0) - 0.2f32,
                length(a, c - 0.3f32, 0.0) - 0.1f32,
            ]),
            abs(z) - 0.7f32,
        ]),
        abs(y + 10f32) - 2f32 - sin(x * 0.1f32),
    ]);
    let t = 8f32 * floor(x / 8f32) + 4f32;
    let h = 20f32 - sin(t) * 10f32;
    union(vec![
        d,
        round_min(vec![
            intersect(vec![
                box3(x - t, y + h * 0.5f32, z + 70f32, 3f32, h, 3f32),
                -box3(
                    abs(x - t) - 1.5f32,
                    modulo(y, 3f32) - 1.5f32,
                    z + 68f32,
                    0.8f32,
                    h * 0.04f32,
                    2f32,
                ),
            ]),
            length(y + 9f32, z + 65f32, 0.0) - 0.5f32
                + value_noise(x, y, z, 5f32, 1f32, 1f32) * 10f32,
            2f32,
        ]),
    ])
}
