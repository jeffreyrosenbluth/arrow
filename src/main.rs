use glam::Vec3;

const HIT_DIST: f32 = 0.001;
const MAX_STEPS: u32 = 32;
const MAX_DIST: f32 = 1000.0;
const EPSILON: f32 = 0.01;
const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;

type MaterialFn = fn(Vec3) -> Material;
// type MaterialFn = Box<dyn Fn(Vec3) -> Material>;

fn v3(value: f32) -> Vec3 {
    Vec3::new(value, value, value)
}

#[derive(Clone, Copy)]
struct Material {
    ambient: Vec3,
    diffuse: Vec3,
    specular: Vec3,
    shininess: f32,
}

struct Surface {
    sd: f32,
    material: MaterialFn,
}

impl Surface {
    fn new(sd: f32, material: MaterialFn) -> Self {
        Self { sd, material }
    }

    fn union(self, other: Self) -> Self {
        if self.sd < other.sd {
            return self;
        };
        return other;
    }
}

type Sdf = Box<dyn Fn(Vec3) -> Surface>;

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

fn sd_sphere(radius: f32, center: Vec3, material: MaterialFn) -> Sdf {
    // let f = Box::new(move |p: Vec3| (p - center).length() - radius);
    // let material = Box::new(move |p: Vec3| material(p));
    Box::new(move |p| Surface::new((p - center).length() - radius, material.clone()))
}

// fn sd_box(p: Vec3, params: &[f32]) -> f32 {
//     let tr = glam::Affine3A::from_rotation_y(-0.4);
//     let tr = tr * glam::Affine3A::from_rotation_x(0.35);
//     let p = tr.transform_point3(p);
//     let b = Vec3::new(params[0], params[1], params[2]);
//     let d = p.abs() - b;
//     d.y.max(d.z).max(d.x).min(0.0) + d.max(Vec3::ZERO).length()
// }

fn normal(p: Vec3, sdf: &Sdf) -> Vec3 {
    let x = Vec3::new(EPSILON, 0.0, 0.0);
    let y = Vec3::new(0.0, EPSILON, 0.0);
    let z = Vec3::new(0.0, 0.0, EPSILON);
    let nx = sdf(p + x).sd - sdf(p - x).sd;
    let ny = sdf(p + y).sd - sdf(p - y).sd;
    let nz = sdf(p + z).sd - sdf(p - z).sd;
    Vec3::new(nx, ny, nz).normalize()
}

fn march(sdf: &Sdf, ro: Vec3, rd: Vec3, light_pos: Vec3) -> Vec3 {
    let mut total_dist = 0.0;
    for _ in 0..MAX_STEPS {
        let p = ro + rd * total_dist;
        let dist = sdf(p).sd;
        if dist < HIT_DIST {
            let n = normal(p, &sdf);
            let material = sdf(p).material;
            return phong((light_pos - p).normalize(), n, rd, &material(p));
        }
        if total_dist > MAX_DIST {
            break;
        }
        total_dist += dist;
    }
    return Vec3::new(0.0, 0.0, 0.0);
}

fn render(sdf: &Sdf, dist_to_camera: f32, light_pos: Vec3) {
    let ro = Vec3::new(0.0, 0.0, -dist_to_camera);
    let mut img = vec![0; (WIDTH * HEIGHT * 3) as usize];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let uv = Vec3::new(
                (x as f32) / (WIDTH as f32),
                (HEIGHT as f32 - y as f32) / (HEIGHT as f32), // flip y
                0.0,
            );
            let rd = Vec3::new(uv.x * 2.0 - 1.0, uv.y * 2.0 - 1.0, 1.0).normalize();
            let col = march(sdf, ro, rd, light_pos);
            let offset = ((y * WIDTH + x) * 3) as usize;
            img[offset + 0] = (col.x * 255.0) as u8;
            img[offset + 1] = (col.y * 255.0) as u8;
            img[offset + 2] = (col.z * 255.0) as u8;
        }
    }
    image::save_buffer("out.png", &img, WIDTH, HEIGHT, image::ColorType::Rgb8).unwrap();
}

fn world() -> Sdf {
    // let displacement = (p.x * 5.0).sin() * (p.y * 5.0).sin() * (p.z * 5.0).sin() * 0.02;
    fn gold(_: Vec3) -> Material {
        Material {
            ambient: 0.5 * Vec3::new(0.7, 0.5, 0.0),
            diffuse: 0.6 * Vec3::new(0.7, 0.7, 0.0),
            specular: 0.6 * Vec3::new(1.0, 1.0, 1.0),
            shininess: 5.,
        }
    }

    fn silver(_: Vec3) -> Material {
        Material {
            ambient: 0.4 * Vec3::new(0.8, 0.8, 0.8),
            diffuse: 0.5 * Vec3::new(0.7, 0.7, 0.7),
            specular: 0.6 * Vec3::new(1.0, 1.0, 1.0),
            shininess: 5.,
        }
    }

    fn checkerboard(p: Vec3) -> Material {
        Material {
            ambient: v3((1.0 + 0.7 * (p.x.floor() + p.z.floor()) % 2.0) * 0.3),
            diffuse: v3(0.3),
            specular: Vec3::ZERO,
            shininess: 0.0,
        }
    }
    let sphere = sd_sphere(1.2, Vec3::ZERO, gold);
    // let cube = sd_box(p, &params[1..=3]);
    // cube.max(-sphere) + displacement
    sphere
}

fn main() {
    render(&world(), 3.0, Vec3::new(2.0, 3.0, -5.0));
}
