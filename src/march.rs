use crate::core::{Light, Material, Sdf};
use glam::{Mat3, Vec3};
use rayon::prelude::*;
use wassily::stipple::poisson_disk;

const MAX_STEPS: u32 = 512;
const MAX_DIST: f32 = 100.0;
const EPSILON: f32 = 0.0001;
const GAMMA_CORRECTION: f32 = 1.0; //0.4545;

fn reflect(i: Vec3, n: Vec3) -> Vec3 {
    i - n * 2.0 * i.dot(n)
}

fn phong(light_dir: Vec3, normal: Vec3, rd: Vec3, material: &Material) -> f32 {
    let ambient = material.ambient;
    let diffuse = material.diffuse * normal.dot(light_dir).clamp(0.0, 1.0);
    let specular = material.specular
        * rd.dot(reflect(light_dir, normal))
            .clamp(0.0, 1.0)
            .powf(material.shininess);
    ambient + diffuse + specular
}

fn normal(p: Vec3, sdf: &Sdf) -> Vec3 {
    let x = Vec3::new(EPSILON, 0.0, 0.0);
    let y = Vec3::new(0.0, EPSILON, 0.0);
    let z = Vec3::new(0.0, 0.0, EPSILON);
    let nx = sdf(p + x).sd - sdf(p - x).sd;
    let ny = sdf(p + y).sd - sdf(p - y).sd;
    let nz = sdf(p + z).sd - sdf(p - z).sd;
    Vec3::new(nx, ny, nz).normalize()
}

fn camera(pos: Vec3, look_at: Vec3) -> Mat3 {
    let forward = (look_at - pos).normalize();
    let right = Vec3::new(0.0, 1.0, 0.0).cross(forward).normalize();
    let up = forward.cross(right).normalize();
    Mat3::from_cols(right, up, forward)
}

fn softshadow(sdf: &Sdf, ro: Vec3, rd: Vec3, mint: f32, maxt: f32, k: f32) -> f32 {
    let mut res: f32 = 1.0;
    let mut t = mint;
    for _ in 0..16 {
        let h = sdf(ro + rd * t).sd;
        if h < EPSILON {
            return 0.0;
        }
        res = res.min(k * h / t);
        t += h;
        if t > maxt {
            return res;
        }
    }
    return res.clamp(0.0, 1.0);
}

fn ambient_occlusion(sdf: &Sdf, p: Vec3, n: Vec3) -> f32 {
    let mut occ: f32 = 0.0;
    let mut w: f32 = 1.0;
    for i in 0..5 {
        let h = 0.01 + 0.03 * i as f32;
        let d = sdf(p + n * h).sd;
        occ += (h - d) * w;
        w *= 0.95;
        if occ > 0.35 {
            break;
        }
    }
    (1.0 - 3.0 * occ).clamp(0.0, 1.0)
}

fn march(sdf: &Sdf, ro: Vec3, rd: Vec3, lights: &[Light], background: f32) -> f32 {
    let mut total_dist = 0.0;
    for _ in 0..MAX_STEPS {
        let p = ro + rd * total_dist;
        let surface = sdf(p);
        let dist = surface.sd;
        if dist.abs() < EPSILON {
            let n = normal(p, &sdf);
            let material = surface.material;
            let mut col = 0.0;
            lights.iter().for_each(|light| {
                col += light.intensity
                    * phong((light.position - p).normalize(), n, rd, &material)
                    * softshadow(sdf, p, (light.position - p).normalize(), 0.1, 1.0, 2.0)
                    * ambient_occlusion(sdf, p, n);
            });
            return col;
        }
        if total_dist > MAX_DIST {
            break;
        }
        total_dist += dist;
    }
    return background;
}

pub fn render_stipple(
    sdf: &Sdf,
    dist_to_camera: f32,
    lights: &[Light],
    background: f32,
    width: u32,
    height: u32,
    anti_aliasing: u32,
) -> Vec<(f32, f32, f32)> {
    let ro = Vec3::new(0.0, 0.0, -dist_to_camera);
    let cam_mat = camera(ro, Vec3::ZERO);
    // let mut img_data: Vec<(f32, f32, f32)> = Vec::with_capacity((width * height) as usize);
    let pts = poisson_disk(width as f32, height as f32, 3.0, 0);
    let img_data = pts.into_par_iter().map(|p| {
        // for p in pts {
        // for y in 0..height {
        //     for x in 0..width {
        let mut col = 0.0;
        for m in 0..anti_aliasing {
            for n in 0..anti_aliasing {
                let ox = (m as f32) / (anti_aliasing as f32) - 0.5;
                let oy = (n as f32) / (anti_aliasing as f32) - 0.5;
                let uv = Vec3::new(
                    (p.x as f32) / (height as f32),
                    (height as f32 - p.y as f32) / (height as f32), // flip y
                    0.0,
                );
                let rd = cam_mat
                    * Vec3::new(
                        (uv.x + ox / height as f32) * 2.0 - 1.0,
                        (uv.y + oy / height as f32) * 2.0 - 1.0,
                        1.0,
                    )
                    .normalize();
                col += march(sdf, ro, rd, lights, background).powf(GAMMA_CORRECTION);
            }
        }
        col /= (anti_aliasing * anti_aliasing) as f32;
        (p.x, p.y, col)
        // img_data.push((p.x, p.y, col))
    });
    // }
    img_data.collect()
}
