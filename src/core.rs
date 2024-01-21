use glam::{Affine3A, Vec3};

pub const I: Affine3A = Affine3A::IDENTITY;
pub const LUM: f32 = 0.4;
pub const SHINE: f32 = 5.0;

pub fn v(value: f32) -> Vec3 {
    Vec3::new(value, value, value)
}

pub fn modulus(a: f32, b: f32) -> f32 {
    ((a % b) + b) % b
}

pub struct Light {
    pub position: Vec3,
    pub intensity: f32,
}

impl Light {
    pub fn new(position: Vec3, intensity: f32) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

// Luminoisty.
pub type Lum = f32;

pub type Sdf = Box<dyn Fn(Vec3) -> f32 + Sync>;

pub fn union(sdf1: Sdf, sdf2: Sdf) -> Sdf {
    Box::new(move |p| sdf1(p).min(sdf2(p)))
}

pub fn intersect(sdf1: Sdf, sdf2: Sdf) -> Sdf {
    Box::new(move |p| sdf1(p).max(sdf2(p)))
}

pub fn difference(sdf1: Sdf, sdf2: Sdf) -> Sdf {
    Box::new(move |p| sdf1(p).max(-sdf2(p)))
}

pub fn perturb(sdf: Sdf, f: fn(Vec3) -> f32) -> Sdf {
    Box::new(move |p| sdf(p) + f(p))
}

pub fn round(sdf: Sdf, radius: f32) -> Sdf {
    Box::new(move |p| sdf(p) - radius)
}

pub fn unions(sdfs: Vec<Sdf>) -> Sdf {
    sdfs.into_iter().reduce(|acc, sdf| union(acc, sdf)).unwrap()
}

pub fn intersects(sdfs: Vec<Sdf>) -> Sdf {
    sdfs.into_iter()
        .reduce(|acc, sdf| intersect(acc, sdf))
        .unwrap()
}
