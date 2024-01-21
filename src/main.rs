use std::vec;

use arrow::core::*;
use arrow::march::render;
use arrow::sdf::*;
use glam::{Affine3A, Vec3};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;

fn displacement(p: Vec3) -> f32 {
    let freq = 10.0;
    (p.x * freq).sin() * (p.y * freq).sin() * (p.z * freq).sin() * 0.1
}

#[allow(dead_code)]
fn scene0() -> Sdf {
    let sphere_gold = perturb(sd_sphere(1.0, Vec3::new(-1.0, 0.0, 0.0), I), displacement);
    let sphere_red = sd_sphere(0.75, Vec3::new(1.0, 0.0, 0.0), I);
    let floor = sd_plane(Vec3::new(0.05, 1.0, 0.0), I);

    let mut tr = Affine3A::from_rotation_y(-0.4);
    tr = tr * Affine3A::from_rotation_x(0.35);
    let cube = sd_round_box(v(0.6), 0.05, Vec3::new(1.0, 0.0, 0.0), tr);

    let tr = Affine3A::from_rotation_x(0.5);
    let torus = sd_torus(0.6, 0.2, Vec3::new(0.1, 0.0, -0.2), tr);

    let capsule = sd_capsule(
        0.25,
        Vec3::new(-0.5, 1.9, 0.1),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        I,
    );

    let rounded_cube = round(
        sd_box(Vec3::new(0.4, 0.3, 0.0), Vec3::new(-1.9, 1.9, 0.0), I),
        0.2,
    );

    let mut balls = Vec::new();
    for i in 0..10 {
        balls.push(sd_sphere(
            0.07,
            Vec3::new(-0.9 + i as f32 * 0.2, -0.9, -1.5),
            I,
        ));
    }

    let frame = difference(cube, sphere_red);

    let cylinder = sd_cylinder(
        0.1,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.75, -0.5, -1.0),
        Vec3::new(1.25, 0.5, -1.5),
        I,
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
    let plane = sd_plane(Vec3::new(0.05, 1.0, 0.0), I);
    union(plane, sd_sphere(1.0, Vec3::new(0.0, 0.0, 0.0), I))
}

fn main() {
    let background = 0.9;
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
    image::save_buffer("out.png", &img_data, WIDTH, HEIGHT, image::ColorType::L8).unwrap();
}
