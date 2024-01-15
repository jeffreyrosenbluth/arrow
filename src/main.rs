use arrow::core::*;
use arrow::march::render;
use arrow::sdf::*;
use glam::{Affine3A, Vec3};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;

fn modulus(a: f32, b: f32) -> f32 {
    ((a % b) + b) % b
}

fn world() -> Sdf {
    fn gold(_: Vec3) -> Material {
        Material::color(Vec3::new(0.7, 0.6, 0.0), 5.0)
    }

    fn rust(_: Vec3) -> Material {
        Material::color(Vec3::new(0.7, 0.2, 0.0), 10.0)
    }

    fn red(_: Vec3) -> Material {
        Material::color(Vec3::new(1.0, 0.0, 0.1), 50.0)
    }

    fn teal(_: Vec3) -> Material {
        Material::color(Vec3::new(0.2, 0.4, 0.4), 1.0)
    }

    fn checkerboard(p: Vec3) -> Material {
        Material {
            ambient: v3(modulus(1.0 + 0.7 * (p.x.floor() + p.z.floor()), 2.0) * 0.3),
            diffuse: v3(0.3),
            specular: Vec3::ZERO,
            shininess: 1.0,
        }
    }

    fn displacement(p: Vec3) -> f32 {
        let freq = 10.0;
        (p.x * freq).sin() * (p.y * freq).sin() * (p.z * freq).sin() * 0.1
    }

    let sphere_gold = perturb(
        sd_sphere(1.0, Vec3::new(-1.0, 0.0, 0.0), gold),
        displacement,
    );
    let sphere_red = sd_sphere(0.75, Vec3::new(1.0, 0.0, 0.0), rust);
    let floor = Box::new(move |p: Vec3| Surface::new(p.y + 1.0, checkerboard));

    let mut tr = Affine3A::from_rotation_y(-0.4);
    tr = tr * Affine3A::from_rotation_x(0.35);
    let cube = sd_round_box(v3(0.6), 0.05, Vec3::new(1.0, 0.0, 0.0), tr, teal);

    let tr = Affine3A::from_rotation_x(0.5);
    let torus = sd_torus(0.6, 0.2, v3(0.0), tr, red);

    union(
        union(union(floor, sphere_gold), difference(cube, sphere_red)),
        torus,
    )
}

fn main() {
    let background = Vec3::new(0.0, 0.0, 0.1);
    let img_data = render(
        &world(),
        3.0,
        Vec3::new(8.0, 6.0, -5.0),
        background,
        WIDTH,
        HEIGHT,
    );
    image::save_buffer("out.png", &img_data, WIDTH, HEIGHT, image::ColorType::Rgb8).unwrap();
}
