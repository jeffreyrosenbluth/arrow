use crate::core::{Material, Sdf};
use glam::{Mat3, Vec3};

const HIT_DIST: f32 = 0.001;
const MAX_STEPS: u32 = 512;
const MAX_DIST: f32 = 1000.0;
const EPSILON: f32 = 0.001;
const GAMMA_CORRECTION: f32 = 0.4545;

fn reflect(i: Vec3, n: Vec3) -> Vec3 {
    i - n * 2.0 * i.dot(n)
}

fn phong(light_dir: Vec3, normal: Vec3, rd: Vec3, material: &Material) -> Vec3 {
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
        if h < HIT_DIST {
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

fn march(sdf: &Sdf, ro: Vec3, rd: Vec3, light_pos: Vec3, background: Vec3) -> Vec3 {
    let mut total_dist = 0.0;
    for _ in 0..MAX_STEPS {
        let p = ro + rd * total_dist;
        let dist = sdf(p).sd;
        if dist < HIT_DIST {
            let n = normal(p, &sdf);
            let material = sdf(p).material;
            return phong((light_pos - p).normalize(), n, rd, &material(p))
                * softshadow(sdf, p, (light_pos - p).normalize(), 0.1, 1.0, 2.0);
        }
        if total_dist > MAX_DIST {
            break;
        }
        total_dist += dist;
    }
    return background;
}

pub fn render(
    sdf: &Sdf,
    dist_to_camera: f32,
    light_pos: Vec3,
    background: Vec3,
    width: u32,
    height: u32,
) -> Vec<u8> {
    let ro = Vec3::new(0.0, 0.0, -dist_to_camera);
    let mut img_data = vec![0; (width * height * 3) as usize];
    for y in 0..height {
        for x in 0..width {
            let uv = Vec3::new(
                (x as f32) / (width as f32),
                (height as f32 - y as f32) / (height as f32), // flip y
                0.0,
            );
            let rd = camera(ro, Vec3::ZERO)
                * Vec3::new(uv.x * 2.0 - 1.0, uv.y * 2.0 - 1.0, 1.0).normalize();
            let col = march(sdf, ro, rd, light_pos, background).powf(GAMMA_CORRECTION);
            let offset = ((y * width + x) * 3) as usize;
            img_data[offset + 0] = (col.x * 255.0) as u8;
            img_data[offset + 1] = (col.y * 255.0) as u8;
            img_data[offset + 2] = (col.z * 255.0) as u8;
        }
    }
    img_data
}
