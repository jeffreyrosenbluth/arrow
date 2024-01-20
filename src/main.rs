use std::vec;

use arrow::core::*;
use arrow::sdf::*;
use arrow::{core::Light, march::render_stipple};
use glam::{Affine2, Affine3A, Vec2, Vec3, Vec3Swizzles};
use wassily::prelude::*;

const WIDTH: u32 = 2048;
const HEIGHT: u32 = 2048;

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
        ambient: grayscale(v3(
            modulus(1.0 + 0.7 * (p.x.floor() + p.z.floor()), 2.0) * 0.3
        )),
        diffuse: 0.3,
        specular: 0.0,
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

// Not working yet, needs to implement the shade function: https://www.shadertoy.com/view/XcS3zK
#[allow(dead_code)]
fn scene2(t: f32) -> Sdf {
    fn rot(a: f32) -> Affine2 {
        Affine2::from_angle(a)
    }
    fn g(p: Vec3) -> f32 {
        let s = Vec3::new(p.y.sin(), p.z.sin(), p.x.sin());
        let c = Vec3::new(p.z.cos(), p.x.cos(), p.y.cos());
        c.dot(s)
    }
    Box::new(move |p| {
        let mut q = p;
        let xz = rot(t).transform_point2(q.xz());
        q.x = xz.x;
        q.z = xz.y;
        let xy = rot(0.3).transform_point2(q.xy());
        q.x = xy.x - 0.5;
        q.y = xy.y - 0.5;
        let mut d = (-(Vec2::new(q.y, q.xz().length() - 2.0)).length() - 1.8 + t.cos() * 0.3).abs();
        let h = g(p.yxz() * 4.0) / 4.0;
        d = Vec2::new(d, h).length() - 0.3;
        let c = Material::color(v3(h), 1.0);
        Surface::new(d, c)
    })
}

fn main() {
    let background = 0.5;
    let img_data = render_stipple(
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
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    canvas.fill(*WHITE);
    // let mut rng = SmallRng::from_entropy();
    for q in img_data {
        let c = 3.5 * (1.0 - q.2).powf(1.5);
        // let b = rng.gen_bool(c as f64);
        // if b {
        // canvas.dot(q.0, q.1, Color::from_rgba(0.0, 0.0, 0.0, 1.0).unwrap());
        Shape::new()
            .circle(pt(q.0, q.1), c)
            .fill_color(*BLACK)
            .no_stroke()
            .draw(&mut canvas);
        // }
    }
    canvas.save_png("out.png");
    // image::save_buffer("out.png", &img_data, WIDTH, HEIGHT, image::ColorType::L8).unwrap();
}
