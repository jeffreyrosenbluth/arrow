use glam::{Affine3A, Mat3, Vec3};

const HIT_DIST: f32 = 0.001;
const MAX_STEPS: u32 = 512;
const MAX_DIST: f32 = 1000.0;
const EPSILON: f32 = 0.001;
const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;

type MaterialFn = fn(Vec3) -> Material;
// type MaterialFn = Box<dyn Fn(Vec3) -> Material>;

fn v3(value: f32) -> Vec3 {
    Vec3::new(value, value, value)
}

fn modulus(a: f32, b: f32) -> f32 {
    ((a % b) + b) % b
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

    fn intersect(self, other: Self) -> Self {
        if self.sd > other.sd {
            return self;
        };
        return other;
    }

    fn difference(self, other: Self) -> Self {
        if self.sd > -other.sd {
            return self;
        };
        return Self::new(-other.sd, other.material);
    }
}

type Sdf = Box<dyn Fn(Vec3) -> Surface>;

fn union(sdf1: Sdf, sdf2: Sdf) -> Sdf {
    Box::new(move |p| sdf1(p).union(sdf2(p)))
}

fn intersect(sdf1: Sdf, sdf2: Sdf) -> Sdf {
    Box::new(move |p| sdf1(p).intersect(sdf2(p)))
}

fn difference(sdf1: Sdf, sdf2: Sdf) -> Sdf {
    Box::new(move |p| sdf1(p).difference(sdf2(p)))
}

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
    Box::new(move |p| Surface::new((p - center).length() - radius, material.clone()))
}

fn sd_box(b: Vec3, transform: Affine3A, material: MaterialFn) -> Sdf {
    // let tr = Affine3A::from_rotation_y(-0.4);
    // let tr = tr * glam::Affine3A::from_rotation_x(0.35);
    Box::new(move |p| {
        let p = transform.transform_point3(p);
        let d = p.abs() - b;
        Surface::new(
            d.y.max(d.z).max(d.x).min(0.0) + d.max(Vec3::ZERO).length(),
            material.clone(),
        )
    })
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

fn render(sdf: &Sdf, dist_to_camera: f32, light_pos: Vec3, background: Vec3) {
    let ro = Vec3::new(0.0, 0.0, -dist_to_camera);
    let mut img = vec![0; (WIDTH * HEIGHT * 3) as usize];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let uv = Vec3::new(
                (x as f32) / (WIDTH as f32),
                (HEIGHT as f32 - y as f32) / (HEIGHT as f32), // flip y
                0.0,
            );
            let rd = camera(ro, Vec3::ZERO)
                * Vec3::new(uv.x * 2.0 - 1.0, uv.y * 2.0 - 1.0, 1.0).normalize();
            let col = march(sdf, ro, rd, light_pos, background);
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
    fn red(_: Vec3) -> Material {
        Material {
            ambient: 0.5 * Vec3::new(0.7, 0.1, 0.0),
            diffuse: 0.6 * Vec3::new(0.7, 0.2, 0.0),
            specular: 0.6 * Vec3::new(1.0, 1.0, 1.0),
            shininess: 10.,
        }
    }

    fn silver(_: Vec3) -> Material {
        Material {
            ambient: 0.8 * Vec3::new(0.2, 0.4, 0.4),
            diffuse: 0.7 * Vec3::new(0.2, 0.4, 0.4),
            specular: 0.1 * Vec3::new(1.0, 1.0, 1.0),
            shininess: 1.0,
        }
    }

    fn checkerboard(p: Vec3) -> Material {
        Material {
            ambient: v3(modulus(1.0 + 0.7 * (p.x.floor() + p.z.floor()), 2.0) * 0.3),
            diffuse: v3(0.3),
            specular: Vec3::ZERO,
            shininess: 1.0,
        }
    }
    let sphere_gold = sd_sphere(1.0, Vec3::new(-1.0, 0.0, 0.0), gold);
    let sphere_black = sd_sphere(0.75, Vec3::new(1.0, 0.0, 0.0), red);
    let floor = Box::new(move |p: Vec3| Surface::new(p.y + 1.0, checkerboard));

    let mut tr = Affine3A::from_rotation_y(-0.4);
    tr = tr * Affine3A::from_rotation_x(0.35);
    tr = tr * Affine3A::from_translation(Vec3::new(-1.0, 0.0, 0.0));
    let cube = sd_box(v3(0.6), tr, silver);

    union(union(floor, sphere_gold), difference(cube, sphere_black))
}

fn main() {
    let background = Vec3::new(0.0, 0.0, 0.15);
    render(&world(), 3.0, Vec3::new(8.0, 6.0, -5.0), background);
}
