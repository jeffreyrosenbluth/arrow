use std::vec;

use arrow::core::*;
use arrow::march::render;
use arrow::sdf::*;
use glam::{Affine3A, Vec3};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;

fn modulus(a: f32, b: f32) -> f32 {
    ((a % b) + b) % b
}

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

fn green(_: Vec3) -> Material {
    Material::color(Vec3::new(0.4, 0.7, 0.1), 50.0)
}

fn magenta(_: Vec3) -> Material {
    Material::color(Vec3::new(0.5, 0.0, 0.5), 20.0)
}

fn slate(_: Vec3) -> Material {
    Material::color(Vec3::new(0.25, 0.35, 0.35), 10.0)
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

#[allow(dead_code)]
fn scene0() -> Sdf {
    let sphere_gold = perturb(
        sd_sphere(1.0, Vec3::new(-1.0, 0.0, 0.0), I, gold),
        displacement,
    );
    let sphere_red = sd_sphere(0.75, Vec3::new(1.0, 0.0, 0.0), I, rust);
    let floor = sd_plane(Vec3::new(0.05, 1.0, 0.0), I, checkerboard);
    // let floor = Box::new(move |p: Vec3| Surface::new(p.y + 1.0, checkerboard));

    let mut tr = Affine3A::from_rotation_y(-0.4);
    tr = tr * Affine3A::from_rotation_x(0.35);
    let cube = sd_round_box(v3(0.6), 0.05, Vec3::new(1.0, 0.0, 0.0), tr, teal);

    let tr = Affine3A::from_rotation_x(0.5);
    let torus = sd_torus(0.6, 0.2, Vec3::new(0.1, 0.0, -0.2), tr, red);

    let capsule = sd_capsule(
        0.25,
        Vec3::new(-0.5, 1.9, 0.1),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        I,
        slate,
    );

    let rounded_cube = round(
        sd_box(
            Vec3::new(0.4, 0.3, 0.0),
            Vec3::new(-1.9, 1.9, 0.0),
            I,
            green,
        ),
        0.2,
    );

    let mut balls = Vec::new();
    for i in 0..10 {
        balls.push(sd_sphere(
            0.07,
            Vec3::new(-0.9 + i as f32 * 0.2, -0.9, -1.5),
            I,
            magenta,
        ));
    }

    let frame = difference(cube, sphere_red);

    let cylinder = sd_cylinder(
        0.1,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.75, -0.5, -1.0),
        Vec3::new(1.25, 0.5, -1.5),
        I,
        rust,
    );

    let a = unions(vec![
        sphere_gold,
        floor,
        torus,
        capsule,
        frame,
        rounded_cube,
        cylinder,
    ]);
    let b = unions(balls);
    union(a, b)
}

#[allow(dead_code)]
fn scene1() -> Sdf {
    let plane = sd_plane(Vec3::new(0.05, 1.0, 0.0), I, slate);
    union(plane, sd_sphere(1.0, Vec3::new(0.0, 0.0, 0.0), I, gold))
}

fn main() {
    let background = Vec3::new(0.0, 0.0, 0.2);
    let img_data = render(
        &scene0(),
        3.25,
        &vec![
            Light::new(Vec3::new(-4.0, 6.0, -6.0), 0.8),
            Light::new(Vec3::new(0.0, 0.0, -6.0), 0.2),
        ],
        background,
        WIDTH,
        HEIGHT,
        2,
    );
    image::save_buffer("out.png", &img_data, WIDTH, HEIGHT, image::ColorType::Rgb8).unwrap();
}
