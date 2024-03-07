use crate::core::{v3, Light, Lum, Sdf, LUM, SHINE};
use glam::{Mat3, Vec3};
use rayon::prelude::*;

const MAX_STEPS: u32 = 128; //512;
const MAX_DIST: f32 = 75.0;
const EPSILON: f32 = 0.001;

fn reflect(i: Vec3, n: Vec3) -> Vec3 {
    i - n * 2.0 * i.dot(n)
}

fn phong(light_dir: Vec3, normal: Vec3, rd: Vec3) -> Lum {
    let diffuse = LUM * normal.dot(light_dir).clamp(0.0, 1.0);
    let specular = LUM
        * rd.dot(reflect(light_dir, normal))
            .clamp(0.0, 1.0)
            .powf(SHINE);
    LUM + diffuse + specular
}

fn normal(p: Vec3, sdf: &Sdf) -> Vec3 {
    let x = v3(EPSILON, 0.0, 0.0);
    let y = v3(0.0, EPSILON, 0.0);
    let z = v3(0.0, 0.0, EPSILON);
    let nx = sdf(p + x) - sdf(p - x);
    let ny = sdf(p + y) - sdf(p - y);
    let nz = sdf(p + z) - sdf(p - z);
    v3(nx, ny, nz).normalize()
}

fn camera(pos: Vec3, look_at: Vec3) -> Mat3 {
    let up = v3(0.0, 1.0, 0.0);
    let w = (look_at - pos).normalize();
    let u = up.cross(w).normalize();
    let v = w.cross(u).normalize();
    Mat3::from_cols(u, v, w)
}

fn softshadow(sdf: &Sdf, ro: Vec3, rd: Vec3, mint: f32, maxt: f32, k: f32) -> f32 {
    let mut res: f32 = 1.0;
    let mut t = mint;
    for _ in 0..16 {
        let h = sdf(ro + rd * t);
        if h < EPSILON {
            return 0.0;
        }
        res = res.min(k * h / t);
        t += h;
        if t > maxt {
            return res;
        }
    }
    res.clamp(0.0, 1.0)
}

fn ambient_occlusion(sdf: &Sdf, p: Vec3, n: Vec3) -> f32 {
    let mut occ: f32 = 0.0;
    let mut w: f32 = 1.0;
    for i in 0..5 {
        let h = 0.01 + 0.03 * i as f32;
        let d = sdf(p + n * h);
        occ += (h - d) * w;
        w *= 0.95;
        if occ > 0.35 {
            break;
        }
    }
    (1.0 - 3.0 * occ).clamp(0.0, 1.0)
}

fn march(sdf: &Sdf, ro: Vec3, rd: Vec3, lights: &[Light], background: Lum) -> Lum {
    let mut total_dist = 0.0;
    for _ in 0..MAX_STEPS {
        let p = ro + rd * total_dist;
        let dist = sdf(p);
        if dist.abs() < EPSILON {
            let n = normal(p, sdf);
            let mut col = 0.0;
            lights.iter().for_each(|light| {
                col += light.intensity
                    * phong((light.position - p).normalize(), n, rd)
                    * softshadow(sdf, p, (light.position - p).normalize(), 0.2, 1.0, 4.0)
                    * ambient_occlusion(&sdf, p, n);
            });
            return col;
        }
        if total_dist > MAX_DIST {
            break;
        }
        total_dist += dist;
    }
    background
}

pub fn render(
    sdf: &Sdf,
    camera_pos: Vec3,
    look_at: Vec3,
    lights: &[Light],
    background: Lum,
    width: u32,
    height: u32,
    anti_aliasing: u32,
) -> Vec<u8> {
    let cam_mat = camera(camera_pos, look_at);
    let mut img_data: Vec<u8> = Vec::with_capacity((width * height) as usize);
    for y in 0..height {
        let scanline: Vec<u8> = (0..width)
            .into_par_iter()
            .map(|x| {
                let mut col = 0.0;
                for m in 0..anti_aliasing {
                    for n in 0..anti_aliasing {
                        let ox = (m as f32) / (anti_aliasing as f32) - 0.5;
                        let oy = (n as f32) / (anti_aliasing as f32) - 0.5;
                        let uv = v3(
                            (2.0 * ((x as f32) + ox) - width as f32) / (height as f32),
                            (2.0 * ((height as f32 - y as f32) + oy) - height as f32)
                                / (height as f32),
                            0.0,
                        );
                        let rd = cam_mat * v3(uv.x, uv.y, 1.0).normalize();
                        col += march(sdf, camera_pos, rd, lights, background);
                    }
                }
                col /= (anti_aliasing * anti_aliasing) as f32;
                (col * 255.0) as u8
            })
            .collect();
        for col in scanline {
            img_data.push(col);
        }
    }
    img_data
}
