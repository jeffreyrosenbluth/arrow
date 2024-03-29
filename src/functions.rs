use glam::{Mat2, Vec2, Vec3};
use std::f32::consts::TAU;

use crate::{
    core::{fbm, v3, I, ZERO3},
    sdf::{sd_box, sd_torus},
};

pub fn sin(x: f32) -> f32 {
    x.sin()
}

pub fn cos(x: f32) -> f32 {
    x.cos()
}

pub fn acos(x: f32) -> f32 {
    x.acos()
}

pub fn asin(x: f32) -> f32 {
    x.asin()
}

pub fn fake_sine(x: f32) -> f32 {
    ((x - x.floor() - 0.5) * 2.0).abs() * x * (6.0 - 4.0 * x) - 1.0
}

pub fn tan(x: f32) -> f32 {
    x.tan()
}

pub fn atan(x: f32) -> f32 {
    x.atan()
}

pub fn atan2(x: f32, y: f32) -> f32 {
    x.atan2(y)
}

pub fn sinh(x: f32) -> f32 {
    x.sinh()
}

pub fn cosh(x: f32) -> f32 {
    x.cosh()
}

pub fn tanh(x: f32) -> f32 {
    x.tanh()
}

pub fn asinh(x: f32) -> f32 {
    x.asinh()
}

pub fn acosh(x: f32) -> f32 {
    x.acosh()
}

pub fn atanh(x: f32) -> f32 {
    x.atanh()
}

pub fn exp(x: f32) -> f32 {
    x.exp()
}

pub fn exp2(x: f32) -> f32 {
    x.exp2()
}

pub fn log(x: f32) -> f32 {
    x.ln()
}

pub fn log2(x: f32) -> f32 {
    x.log2()
}

pub fn pow(x: f32, p: f32) -> f32 {
    x.powf(p)
}

pub fn sqrt(x: f32) -> f32 {
    x.sqrt()
}

pub fn abs(x: f32) -> f32 {
    x.abs()
}

pub fn sign(x: f32) -> f32 {
    x.signum()
}

pub fn floor(x: f32) -> f32 {
    x.floor()
}

pub fn ceil(x: f32) -> f32 {
    x.ceil()
}

pub fn trunc(x: f32) -> f32 {
    x.trunc()
}

pub fn fract(x: f32) -> f32 {
    x.fract()
}

pub fn round(x: f32) -> f32 {
    x.round()
}

pub fn modulo(a: f32, b: f32) -> f32 {
    ((a % b) + b) % b
}

pub fn min(x: f32, y: f32) -> f32 {
    x.min(y)
}

pub fn max(x: f32, y: f32) -> f32 {
    x.max(y)
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    x.max(min).min(max)
}

pub fn mix(x: f32, y: f32, a: f32) -> f32 {
    x * (1.0 - a) + y * a
}

pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

pub fn length(x: f32, y: f32, z: f32) -> f32 {
    v3(x, y, z).length()
}

pub fn distance(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> f32 {
    v3(x1, y1, z1).distance(v3(x2, y2, z2))
}

pub fn dot(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> f32 {
    v3(x1, y1, z1).dot(v3(x2, y2, z2))
}

pub fn cross(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> Vec3 {
    v3(x1, y1, z1).cross(v3(x2, y2, z2))
}

pub fn normalize(x: f32, y: f32, z: f32) -> Vec3 {
    v3(x, y, z).normalize()
}

pub fn union(xs: Vec<f32>) -> f32 {
    *xs.iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Less))
        .unwrap()
}

pub fn round_min(xs: Vec<f32>) -> f32 {
    let mut xs = xs;
    let r = xs.pop().unwrap();
    let d = xs.into_iter().reduce(|a, b| smooth_min(a, b, r));
    d.unwrap_or(r)
}

pub fn round_max(xs: Vec<f32>) -> f32 {
    let mut xs = xs;
    let r = xs.pop().unwrap();
    let d = xs.into_iter().reduce(|a, b| smooth_max(a, b, r));
    d.unwrap_or(r)
}

pub fn intersect(xs: Vec<f32>) -> f32 {
    *xs.iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Greater))
        .unwrap()
}

pub fn add_mul(x: f32, y: f32, z: f32, a: f32, b: f32, c: f32, t: f32) -> Vec3 {
    v3(x + a * t, y + b * t, z + c * t)
}

pub fn smooth_abs(x: f32, p: f32) -> f32 {
    (x * x + p).sqrt()
}

pub fn smooth_min(a: f32, b: f32, r: f32) -> f32 {
    if a < r && b < r {
        r - Vec2::new(r - a, r - b).length()
    } else {
        a.min(b)
    }
}

pub fn smooth_max(a: f32, b: f32, r: f32) -> f32 {
    if -a < r && -b < r {
        r - Vec2::new(r + a, r + b).length()
    } else {
        a.max(b)
    }
}

pub fn poly_smooth_abs(x: f32, m: f32) -> f32 {
    if x.abs() > m {
        x
    } else {
        (2.0 - x / m) * x * x / m
    }
}
pub fn smooth_clamp(x: f32, p: f32, a: f32, b: f32) -> f32 {
    (smooth_abs(x - a, p) - smooth_abs(x - b, p) + a + b) / 2.0
}

pub fn poly_smooth_clamp(x: f32, p: f32, a: f32, b: f32) -> f32 {
    (poly_smooth_abs(x - a, p) - poly_smooth_abs(x - b, p) + a + b) / 2.0
}

pub fn torus(x: f32, y: f32, z: f32, r1: f32, r2: f32) -> f32 {
    let p = v3(x, y, z);
    let sdf = sd_torus(r1, r2, ZERO3, I);
    sdf(p)
}

pub fn value_noise(x: f32, y: f32, z: f32, scale: f32, offset: f32, octaves: f32) -> f32 {
    fbm(x, y, z, scale, offset, octaves as u32)
}

pub fn hash(x: f32, y: f32, z: f32) -> f32 {
    crate::core::hash(v3(x, y, z))
}

pub fn box2(x: f32, y: f32, a: f32, b: f32) -> f32 {
    let x = x.abs() - a;
    let y = y.abs() - b;
    if x > 0.0 && y > 0.0 {
        v3(x, y, 0.0).length()
    } else {
        x.max(y)
    }
}

pub fn box3(x: f32, y: f32, z: f32, a: f32, b: f32, c: f32) -> f32 {
    let p = v3(x, y, z);
    let b = v3(a, b, c);
    let sdf = sd_box(b, ZERO3, I);
    sdf(p)
}

pub fn rot0(x: f32, y: f32) -> [f32; 2] {
    let v = Vec2::new(x, y);
    let a = 0.1 * TAU;
    let m = Mat2::from_angle(a);
    let result = m * v;
    [result.x, result.y]
}

pub fn rot1(x: f32, y: f32) -> [f32; 2] {
    let v = Vec2::new(x, y);
    let a = 0.2 * TAU;
    let m = Mat2::from_angle(a);
    let result = m * v;
    [result.x, result.y]
}

pub fn rot(x: f32, y: f32, c: f32, s: f32) -> Vec2 {
    Vec2::new(c * x + s * y, c * y - s * x)
}

pub fn triangle(x: f32) -> f32 {
    (x - (x / 4.0).floor() * 4.0 - 2.0).abs() - 1.0
}

pub fn corner(x: f32, y: f32) -> f32 {
    if x > 0.0 && y > 0.0 {
        v3(x, y, 0.0).length()
    } else {
        x.max(y)
    }
}
