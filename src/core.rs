use glam::{Affine3A, Vec3};

pub const I: Affine3A = Affine3A::IDENTITY;
pub const LUM: f32 = 0.3;
pub const SHINE: f32 = 5.0;
pub const ZERO3: Vec3 = Vec3::ZERO;

pub fn v(value: f32) -> Vec3 {
    Vec3::new(value, value, value)
}

pub fn v3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
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

pub fn smooth_union(sdf1: Sdf, sdf2: Sdf, k: f32) -> Sdf {
    Box::new(move |p| {
        let d1 = sdf1(p);
        let d2 = sdf2(p);
        let h = (k - (d1 - d2).abs()).max(0.0);
        d1.min(d2) - h * h * 0.25 / k
    })
}

pub fn intersect(sdf1: Sdf, sdf2: Sdf) -> Sdf {
    Box::new(move |p| sdf1(p).max(sdf2(p)))
}

pub fn smooth_intersection(sdf1: Sdf, sdf2: Sdf, k: f32) -> Sdf {
    Box::new(move |p| {
        let d1 = sdf1(p);
        let d2 = sdf2(p);
        let h = (k - (d1 - d2).abs()).max(0.0);
        d1.max(d2) + h * h * 0.25 / k
    })
}

pub fn difference(sdf1: Sdf, sdf2: Sdf) -> Sdf {
    Box::new(move |p| sdf1(p).max(-sdf2(p)))
}

pub fn smooth_difference(sdf1: Sdf, sdf2: Sdf, k: f32) -> Sdf {
    Box::new(move |p| {
        let d1 = sdf1(p);
        let d2 = sdf2(p);
        let h = (k - (-d1 - d2).abs()).max(0.0);
        d2.max(-d1) + h * h * 0.25 / k
    })
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

pub fn repeat_x(sdf: Sdf, space: f32) -> Sdf {
    Box::new(move |p| {
        let mut r = p;
        r.x = p.x - space * (p.x / space).round();
        sdf(r)
    })
}

pub fn repeat_y(sdf: Sdf, space: f32) -> Sdf {
    Box::new(move |p| {
        let mut r = p;
        r.y = p.y - space * (p.y / space).round();
        sdf(r)
    })
}
