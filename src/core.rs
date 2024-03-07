use ::noise::{Fbm, NoiseFn, Perlin};
use glam::{Affine3A, Vec3};
// use serde::de;

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

pub fn modulo(a: f32, b: f32) -> f32 {
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

pub fn map_xyz(sdf: Sdf, f: fn(Vec3) -> Vec3) -> Sdf {
    Box::new(move |p| sdf(f(p)))
}

pub fn map_x(sdf: Sdf, f: fn(f32) -> f32) -> Sdf {
    Box::new(move |p| sdf(v3(f(p.x), p.y, p.z)))
}

pub fn map_y(sdf: Sdf, f: fn(f32) -> f32) -> Sdf {
    Box::new(move |p| sdf(v3(p.x, f(p.y), p.z)))
}

pub fn map_z(sdf: Sdf, f: fn(f32) -> f32) -> Sdf {
    Box::new(move |p| sdf(v3(p.x, p.y, f(p.z))))
}

pub fn mirror_x(sdf: Sdf) -> Sdf {
    map_x(sdf, |x| x.abs())
}

pub fn mirror_y(sdf: Sdf) -> Sdf {
    map_y(sdf, |x| x.abs())
}

pub fn perturb<F>(sdf: Sdf, f: F) -> Sdf
where
    F: Fn(Vec3) -> f32 + Sync + Send + 'static,
{
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
    Box::new(move |p| sdf(v3(p.x - space * (p.x / space).round(), p.y, p.z)))
}

pub fn repeat_y(sdf: Sdf, space: f32) -> Sdf {
    Box::new(move |p| sdf(v3(p.x, p.y - space * (p.y / space).round(), p.z)))
}

pub fn repeat_z(sdf: Sdf, space: f32) -> Sdf {
    Box::new(move |p| sdf(v3(p.x, p.y, p.z - space * (p.z / space).round())))
}

pub fn mix(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

pub struct Noise {
    octaves: u32,
    frequency: f32,
    amplitude: f32,
    nf: Fbm<Perlin>,
}

impl Noise {
    pub fn new(octaves: u32, frequency: f32, amplitude: f32) -> Self {
        let nf = Fbm::<Perlin>::new(0);
        Self {
            octaves,
            frequency,
            amplitude,
            nf,
        }
    }

    pub fn get(&self, p: Vec3) -> f32 {
        let mut value = 0.0;
        let mut amplitude = self.amplitude as f64;
        let mut frequency = self.frequency as f64;
        for _ in 0..self.octaves {
            value += amplitude
                * self.nf.get([
                    p.x as f64 * frequency,
                    p.y as f64 * frequency,
                    p.z as f64 * frequency,
                ]);
            amplitude *= 0.5;
            frequency *= 2.0;
        }
        0.5 * value as f32
    }
}

pub fn hash(p: Vec3) -> f32 // replace this by something better
{
    let mut p = (p * 0.3183099 + 0.1).fract();
    p *= 17.0;
    (p.x * p.y * p.z * (p.x + p.y + p.z)).fract()
}

// Return a value between -1 and 1.
pub fn noise(x: Vec3) -> f32 {
    let i: Vec3 = x.floor();
    let f: Vec3 = x.fract();
    let f = f * f * (3.0 - 2.0 * f);

    mix(
        mix(
            mix(
                hash(i + v3(0.0, 0.0, 0.0)),
                hash(i + v3(1.0, 0.0, 0.0)),
                f.x,
            ),
            mix(
                hash(i + v3(0.0, 1.0, 0.0)),
                hash(i + v3(1.0, 1.0, 0.0)),
                f.x,
            ),
            f.y,
        ),
        mix(
            mix(
                hash(i + v3(0.0, 0.0, 1.0)),
                hash(i + v3(1.0, 0.0, 1.0)),
                f.x,
            ),
            mix(
                hash(i + v3(0.0, 1.0, 1.0)),
                hash(i + v3(1.0, 1.0, 1.0)),
                f.x,
            ),
            f.y,
        ),
        f.z,
    ) * 2.0
        - 1.0
}

// The range of the noise depends on the number of octaves.
pub fn fbm(x: f32, y: f32, z: f32, scale: f32, offset: f32, octaves: u32) -> f32 {
    let mut p = v3(x, y, z) * scale + v3(offset, offset, offset);
    let mut a = 0.0;
    let mut sum = 0.0;
    for o in 1..=octaves {
        a += 1.0 / (2.0 * o as f32);
        p *= 2.03;
        sum += a * noise(p);
    }
    sum
}

mod tests {
    use super::*;

    #[test]
    fn test_noise() {
        for i in 0..100 {
            let w = i as f32;
            let n = fbm(w / 2.375, w / 11.8, w / 20.73, 0.03, 0.0, 4);
            println!("Noise: {}", n);
        }
    }
}
