use crate::core::{fbm_perlin, fbm_value, v3, I, ZERO3};
use crate::sdf::sd_box;
use crate::{box2, box3, dot, functions::*, length, perlin_noise, value_noise};
use glam::{Vec2, Vec3};

pub struct Scene {
    pub sdf: fn(Vec3) -> f32,
    pub camera: Vec3,
}

pub fn scene(sdf_name: &'static str) -> Scene {
    match sdf_name {
        "cross" => Scene {
            sdf: cross,
            camera: v3(-10.0, 30.0, -15.0),
        },
        "box_of_balls" => Scene {
            sdf: box_of_balls,
            camera: v3(0.0, 0.0, -20.0),
        },
        "sponge" => Scene {
            sdf: sponge,
            camera: v3(-20.0, 20.0, -5.0),
        },
        "donuts" => Scene {
            sdf: donuts,
            camera: v3(0.0, 0.0, -20.0),
        },
        "desire" => Scene {
            sdf: desire,
            camera: v3(0.0, 0.0, -20.0),
        },
        "apollonius" => Scene {
            sdf: apollonius,
            camera: v3(20.0, 0.0, -60.0),
        },
        "hyperplane" => Scene {
            sdf: hyperplane,
            camera: v3(20.0, 0.0, -60.0),
        },
        "singularity" => Scene {
            sdf: singularity,
            camera: v3(0.0, 0.0, -60.0),
        },
        "mycelia" => Scene {
            sdf: mycelia,
            camera: v3(10.0, 10.0, -15.0),
        },
        "else" => Scene {
            sdf: els,
            camera: v3(0.0, 20.0, -20.0),
        },
        "gnarl" => Scene {
            sdf: gnarl,
            camera: v3(0.0, 0.0, -50.0),
        },
        "system" => Scene {
            sdf: system,
            camera: v3(0.0, 10.0, -50.0),
        },
        "temple" => Scene {
            sdf: temple,
            camera: v3(0.0, -5.0, -40.0),
        },
        "toy" => Scene {
            sdf: toy,
            camera: v3(-0.5, -5.0, -10.0),
        },
        "ghost" => Scene {
            sdf: ghost,
            camera: v3(-10.0, 2.0, -40.0),
        },
        "shai_hulud" => Scene {
            sdf: shai_hulud,
            camera: v3(-8.0, 5.0, 30.0),
        },
        "plato" => Scene {
            sdf: plato,
            camera: v3(0.0, 30.0, -10.0),
        },
        "pawns" => Scene {
            sdf: pawns,
            camera: v3(5.0, 8.0, -20.0),
        },
        "asurf" => Scene {
            sdf: asurf,
            camera: v3(2.0, 5.0, -1.0),
        },
        _ => panic!("Unknown scene: {}", sdf_name),
    }
}

// Camera: v3(-10.0, 30.0, -15.0)
pub fn cross(p: Vec3) -> f32 {
    let Vec3 { x, y, z } = p;
    union(vec![
        box3!(modulo(x, 4f32) - 2f32, y, z, 6f32),
        box3!(x, y, modulo(x, 4f32) - 2f32, 6f32),
        length!(triangle(x), y) - 1f32,
        length!(x + 20f32, y - 20f32, z - 20f32) - 8f32,
    ])
}

// Camera: v3(0.0, 0.0, -20.0)
pub fn box_of_balls(p: Vec3) -> f32 {
    let Vec3 { x, y, z } = p;
    let (a0, a1) = (0.1, 0.2);
    let s = 1f32;
    let [x, y] = rot0(x, y, a0);
    let [x, z] = rot0(x, z, a1);
    let x = abs(x * 2f32) - 8f32;
    let y = abs(y * 2f32) - 8f32;
    let z = abs(z * 2f32) - 8f32;
    let s = s * 0.5f32;
    let [x, y] = rot0(x, y, a0);
    let [x, z] = rot0(x, z, a1);
    let x = abs(x * 2f32) - 8f32;
    let y = abs(y * 2f32) - 8f32;
    let z = abs(z * 2f32) - 8f32;
    let s = s * 0.5f32;
    let [x, y] = rot0(x, y, a0);
    let [x, z] = rot0(x, z, a1);
    let x = abs(x * 2f32) - 8f32;
    let y = abs(y * 2f32) - 8f32;
    let z = abs(z * 2f32) - 8f32;
    let s = s * 0.5f32;
    let [x, y] = rot0(x, y, a0);
    let [x, z] = rot0(x, z, a1);
    let x = abs(x * 2f32) - 8f32;
    let y = abs(y * 2f32) - 8f32;
    let z = abs(z * 2f32) - 8f32;
    let s = s * 0.5f32;
    let [x, y] = rot0(x, y, a0);
    let [x, z] = rot0(x, z, a1);
    let x = abs(x * 2f32) - 8f32;
    let y = abs(y * 2f32) - 8f32;
    let z = abs(z * 2f32) - 8f32;
    let s = s * 0.5f32;
    (length!(x, y, z) - 8f32) * s
}

// Camera: v3(-20.0, 20.0, -5.0)
pub fn sponge(p: Vec3) -> f32 {
    let Vec3 { x, y, z } = p;
    let r = box3!(x, y, z, 9f32, 9f32, 9f32);
    let s = 1f32;
    let x = (modulo(x + 9f32, 18f32) - 9f32) * 3f32;
    let y = (modulo(y + 9f32, 18f32) - 9f32) * 3f32;
    let z = (modulo(z + 9f32, 18f32) - 9f32) * 3f32;
    let s = s / 3f32;
    let r = corner(
        r,
        -union(vec![
            box2!(x, y, 9f32),
            box2!(y, z, 9f32),
            box2!(z, x, 9f32),
        ]) * s,
    );
    let x = (modulo(x + 9f32, 18f32) - 9f32) * 3f32;
    let y = (modulo(y + 9f32, 18f32) - 9f32) * 3f32;
    let z = (modulo(z + 9f32, 18f32) - 9f32) * 3f32;
    let s = s / 3f32;
    let r = corner(
        r,
        -union(vec![
            box2!(x, y, 9f32),
            box2!(y, z, 9f32),
            box2!(z, x, 9f32),
        ]) * s,
    );
    let x = (modulo(x + 9f32, 18f32) - 9f32) * 3f32;
    let y = (modulo(y + 9f32, 18f32) - 9f32) * 3f32;
    let z = (modulo(z + 9f32, 18f32) - 9f32) * 3f32;
    let s = s / 3f32;
    let r = corner(
        r,
        -union(vec![
            box2!(x, y, 9f32),
            box2!(y, z, 9f32),
            box2!(z, x, 9f32),
        ]) * s,
    );
    r
}

pub fn donuts(p: Vec3) -> f32 {
    let Vec3 { x, y, z } = p;
    torus(x, y - 3f32, modulo(z, 8f32) - 4f32, 8f32, 1f32)
}

// Camera: v3(0.0, 0.0, -20.0)
// XXX Not working, might be a0/a1 or camera.
pub fn desire(p: Vec3) -> f32 {
    let Vec3 { x, y, z } = p;
    let (a0, a1) = (0.1, 0.2);
    let [x, y] = rot0(x, y - 1f32, a0);
    let [x, z] = rot0(x, z, a1);
    let yb = abs(y) - 22.5f32;
    union(vec![
        round_max(vec![
            32f32 - corner(-y - 13f32, z - 15f32),
            triangle(x * 0.25f32) * 4f32 - 2f32 + 4f32 * smoothstep(0f32, 16f32, x),
            4f32,
        ]),
        round_max(vec![
            abs(abs(length!(length!(x, z) - 16f32, yb - clamp(yb, -8.5f32, 8.5f32)) - 8f32) - 4f32)
                - 2f32,
            abs(
                abs(length!(abs(x) - 15f32, abs(abs(y) - 15f32) - 15f32, abs(z) - 15f32) - 9f32)
                    - 4f32,
            ) - 2f32,
            1f32,
        ]),
    ])
}

// Camera: v3(0.0, 0.0, -30.0)
pub fn apollonius(p: Vec3) -> f32 {
    let Vec3 { x, y, z } = p;
    let a0 = 0.1;
    let s = 2.5f32;
    let h = s / 2f32;
    let d = (s + h) / 2f32;
    let q = 20f32;
    let y = y - 10f32;
    let [x, y] = rot0(x, y, a0);
    let x = x / q;
    let y = y / q;
    let z = z / q;
    let c = 1f32;
    let x = modulo(x - h, s) - h;
    let y = modulo(y - h, s) - h;
    let z = modulo(z - h, s) - h;
    let t = d / dot!(x, y, z, x, y, z);
    let x = x * t;
    let y = y * t;
    let z = z * t;
    let c = c * t;
    let x = modulo(x - h, s) - h;
    let y = modulo(y - h, s) - h;
    let z = modulo(z - h, s) - h;
    let t = d / dot!(x, y, z, x, y, z);
    let x = x * t;
    let y = y * t;
    let z = z * t;
    let c = c * t;
    let x = modulo(x - h, s) - h;
    let y = modulo(y - h, s) - h;
    let z = modulo(z - h, s) - h;
    let t = d / dot!(x, y, z, x, y, z);
    let x = x * t;
    let y = y * t;
    let z = z * t;
    let c = c * t;
    let x = modulo(x - h, s) - h;
    let y = modulo(y - h, s) - h;
    let z = modulo(z - h, s) - h;
    let t = d / dot!(x, y, z, x, y, z);
    let x = x * t;
    let y = y * t;
    let z = z * t;
    let c = c * t;
    let x = modulo(x - h, s) - h;
    let y = modulo(y - h, s) - h;
    let z = modulo(z - h, s) - h;
    let t = d / dot!(x, y, z, x, y, z);
    let x = x * t;
    let y = y * t;
    let z = z * t;
    let c = c * t;
    let x = modulo(x - h, s) - h;
    let y = modulo(y - h, s) - h;
    let z = modulo(z - h, s) - h;
    let t = d / dot!(x, y, z, x, y, z);
    let x = x * t;
    let y = y * t;
    let z = z * t;
    let c = c * t;
    let x = modulo(x - h, s) - h;
    let y = modulo(y - h, s) - h;
    let z = modulo(z - h, s) - h;
    let t = d / dot!(x, y, z, x, y, z);
    let x = x * t;
    let y = y * t;
    let z = z * t;
    let c = c * t;
    length!(x, y, z) / c * 2f32 - 0.025f32
}

// Camera: v3(20.0, 0.0, -60.0)
pub fn hyperplane(p: Vec3) -> f32 {
    let Vec3 { x, y, z } = p;
    let (a0, a1) = (0.1, 0.2);
    let a = (2f32 * x - 3f32 * z + 6f32 * y) / 7f32;
    let b = (7f32 * x - 2f32 * z + 26f32 * y) / 27f32;
    let c = (6f32 * z - 3f32 * x + 22f32 * y) / 23f32;
    let [x, z] = rot0(x, z + 8f32, a1);
    let [x, y] = rot0(x, y, a0);
    let y = y - 3f32;
    let zz = fract(z / 26f32 - 0.55f32) * 26f32 - 13f32;
    let d = smoothstep(9f32, -12f32, y + 3f32 - z * 0.3f32);
    union(vec![
        corner(
            corner(
                corner(
                    box3!(x, y - 5f32, zz, 7f32, 14f32, 7f32) - 1f32,
                    d - abs(triangle(a)),
                ),
                d - abs(triangle(b)),
            ),
            d - abs(triangle(c)),
        ),
        length!(x + 99f32, y + 445f32, z + 32f32) - 434f32,
    ])
}

pub fn singularity(p: Vec3) -> f32 {
    let Vec3 { x, y, z } = p;
    let a0 = 0.1;
    let [x, z] = rot0(x, z, a0);
    let l = length!(x, y, z);
    let n = 2f32 * value_noise!(atan2(z, x), acos(y / l), l, 0.3f32, 0f32);
    let d = l - 20f32 + n * 5f32;
    let d = abs(d) - 5f32;
    let d = abs(d) - 1f32;
    let b = 99f32;
    let b = union(vec![
        b,
        box2!(
            x,
            y - n * 3f32 + 10f32 - 1f32 * 10f32,
            100f32,
            2f32 + 1f32 * 0.5f32
        ),
    ]);
    let b = union(vec![
        b,
        box2!(
            x,
            y - n * 3f32 + 10f32 - 2f32 * 10f32,
            100f32,
            2f32 + 2f32 * 0.5f32
        ),
    ]);
    let b = union(vec![
        b,
        box2!(
            x,
            y - n * 3f32 + 10f32 - 3f32 * 10f32,
            100f32,
            2f32 + 3f32 * 0.5f32
        ),
    ]);
    let b = union(vec![
        b,
        box2!(
            x,
            y - n * 3f32 + 10f32 - 4f32 * 10f32,
            100f32,
            2f32 + 4f32 * 0.5f32
        ),
    ]);
    round_min(vec![
        y + 20f32 - abs(n) * 0.2f32,
        corner(0f32 - b, d) - 0.4f32,
        20f32,
    ])
}

// Camera: v3(10.0, 10.0, -15.0)
pub fn mycelia(p: Vec3) -> f32 {
    let Vec3 { x, y, z } = p;
    let x = x / 20f32;
    let y = y / 20f32;
    let z = z / 20f32;
    let xm = 0.9f32;
    let ym = 0.3f32;
    let zm = 0.7f32;
    let x = abs(x) - xm;
    let y = abs(y) - ym;
    let z = abs(z) - zm;
    let s = 1f32 / smooth_clamp(length!(x, y, z).powf(3f32), 0.1f32, 0.1f32, 1f32);
    let x = x * s - ym;
    let y = y * s - zm;
    let z = z * s - xm;
    let x = abs(x) - xm;
    let y = abs(y) - ym;
    let z = abs(z) - zm;
    let s = 1f32 / smooth_clamp(length!(x, y, z).powf(3f32), 0.1f32, 0.1f32, 1f32);
    let x = x * s - ym;
    let y = y * s - zm;
    let z = z * s - xm;
    let x = abs(x) - xm;
    let y = abs(y) - ym;
    let z = abs(z) - zm;
    let s = 1f32 / smooth_clamp(length!(x, y, z).powf(3f32), 0.1f32, 0.1f32, 1f32);
    let x = x * s - ym;
    let y = y * s - zm;
    let z = z * s - xm;
    let x = abs(x) - xm;
    let y = abs(y) - ym;
    let z = abs(z) - zm;
    let s = 1f32 / smooth_clamp(length!(x, y, z).powf(3f32), 0.1f32, 0.1f32, 1f32);
    let y = y * s - zm;
    let z = z * s - xm;
    length!(z, y) - 0.1f32
}

// Camera: v3(0.0, 20.0, -20.0)
pub fn els(p: Vec3) -> f32 {
    let Vec3 { x, y, z } = p;
    let y = y - 1f32;
    let r = box3!(x, y, z, 9f32, 9f32, 9f32) - 2f32;
    let s = 1f32;
    let ti = union(vec![
        length!(x, y) - 0.6f32,
        length!(y, z) - 0.6f32,
        length!(z, x) - 0.6f32,
    ]);
    let x = (modulo(x + 9f32, 18f32) - 9f32) * 3f32;
    let y = (modulo(y + 9f32, 18f32) - 9f32) * 3f32;
    let z = (modulo(z + 9f32, 18f32) - 9f32) * 3f32;
    let s = s / 3f32;
    let r = corner(
        r + s,
        -union(vec![
            length!(x, y) - 12f32,
            length!(y, z) - 12f32,
            length!(z, x) - 12f32,
        ]) * s,
    ) - s;
    let x = (modulo(x + 9f32, 18f32) - 9f32) * 3f32;
    let y = (modulo(y + 9f32, 18f32) - 9f32) * 3f32;
    let z = (modulo(z + 9f32, 18f32) - 9f32) * 3f32;
    let s = s / 3f32;
    let r = corner(
        r + s,
        -union(vec![
            length!(x, y) - 12f32,
            length!(y, z) - 12f32,
            length!(z, x) - 12f32,
        ]) * s,
    ) - s;
    let x = (modulo(x + 9f32, 18f32) - 9f32) * 3f32;
    let y = (modulo(y + 9f32, 18f32) - 9f32) * 3f32;
    let z = (modulo(z + 9f32, 18f32) - 9f32) * 3f32;
    let s = s / 3f32;
    let r = corner(
        r + s,
        -union(vec![
            length!(x, y) - 12f32,
            length!(y, z) - 12f32,
            length!(z, x) - 12f32,
        ]) * s,
    ) - s;
    let x = (modulo(x + 9f32, 18f32) - 9f32) * 3f32;
    let y = (modulo(y + 9f32, 18f32) - 9f32) * 3f32;
    let z = (modulo(z + 9f32, 18f32) - 9f32) * 3f32;
    let s = s / 3f32;
    let r = corner(
        r + s,
        -union(vec![
            length!(x, y) - 12f32,
            length!(y, z) - 12f32,
            length!(z, x) - 12f32,
        ]) * s,
    ) - s;
    union(vec![r, ti])
}

pub fn gnarl(p: Vec3) -> f32 {
    let Vec3 { x, y, z } = p;
    let p = abs(y - 18f32) - 13f32;
    let n = value_noise!(x, y, z, 0.03f32, 0.29731, 2f32) * 2f32;
    modulo(p, 12f32 + n * z) - 1.8f32
}

pub fn system(p: Vec3) -> f32 {
    let Vec3 { x, y, z } = p;
    let a0 = 0.2;
    let pi = 3.1415f32;
    let [z, y] = rot0(z, y, a0);
    let z = z + 10f32;
    let r = sqrt(x * x + y * y);
    let t = atan2(y, x) / 2f32 * pi;
    let d = 10000f32;
    let d = min(
        box3!(
            r - 2f32 * 1f32,
            t,
            z,
            0.8f32,
            1f32,
            if floor(modulo(
                1f32 * 1.2f32 + t * mix(1f32, 200f32, pow(hash(1f32, 1f32, 1f32) + 0.5f32, 2f32)),
                3f32
            )) > 0f32
            {
                -1f32
            } else {
                1f32
            }
        ) - 0.2f32,
        d,
    );
    let d = min(
        box3!(
            r - 2f32 * 2f32,
            t,
            z,
            0.8f32,
            1f32,
            if floor(modulo(
                2f32 * 1.2f32 + t * mix(1f32, 200f32, pow(hash(2f32, 2f32, 2f32) + 0.5f32, 2f32)),
                3f32
            )) > 0f32
            {
                -1f32
            } else {
                1f32
            }
        ) - 0.2f32,
        d,
    );
    let d = min(
        box3!(
            r - 2f32 * 3f32,
            t,
            z,
            0.8f32,
            1f32,
            if floor(modulo(
                3f32 * 1.2f32 + t * mix(1f32, 200f32, pow(hash(3f32, 3f32, 3f32) + 0.5f32, 2f32)),
                3f32
            )) > 0f32
            {
                -1f32
            } else {
                1f32
            }
        ) - 0.2f32,
        d,
    );
    let d = min(
        box3!(
            r - 2f32 * 4f32,
            t,
            z,
            0.8f32,
            1f32,
            if floor(modulo(
                4f32 * 1.2f32 + t * mix(1f32, 200f32, pow(hash(4f32, 4f32, 4f32) + 0.5f32, 2f32)),
                3f32
            )) > 0f32
            {
                -1f32
            } else {
                1f32
            }
        ) - 0.2f32,
        d,
    );
    let d = min(
        box3!(
            r - 2f32 * 5f32,
            t,
            z,
            0.8f32,
            1f32,
            if floor(modulo(
                5f32 * 1.2f32 + t * mix(1f32, 200f32, pow(hash(5f32, 5f32, 5f32) + 0.5f32, 2f32)),
                3f32
            )) > 0f32
            {
                -1f32
            } else {
                1f32
            }
        ) - 0.2f32,
        d,
    );
    let d = min(
        box3!(
            r - 2f32 * 6f32,
            t,
            z,
            0.8f32,
            1f32,
            if floor(modulo(
                6f32 * 1.2f32 + t * mix(1f32, 200f32, pow(hash(6f32, 6f32, 6f32) + 0.5f32, 2f32)),
                3f32
            )) > 0f32
            {
                -1f32
            } else {
                1f32
            }
        ) - 0.2f32,
        d,
    );
    let d = min(
        box3!(
            r - 2f32 * 7f32,
            t,
            z,
            0.8f32,
            1f32,
            if floor(modulo(
                7f32 * 1.2f32 + t * mix(1f32, 200f32, pow(hash(7f32, 7f32, 7f32) + 0.5f32, 2f32)),
                3f32
            )) > 0f32
            {
                -1f32
            } else {
                1f32
            }
        ) - 0.2f32,
        d,
    );
    let d = min(
        box3!(
            r - 2f32 * 8f32,
            t,
            z,
            0.8f32,
            1f32,
            if floor(modulo(
                8f32 * 1.2f32 + t * mix(1f32, 200f32, pow(hash(8f32, 8f32, 8f32) + 0.5f32, 2f32)),
                3f32
            )) > 0f32
            {
                -1f32
            } else {
                1f32
            }
        ) - 0.2f32,
        d,
    );
    let d = min(
        box3!(
            r - 2f32 * 9f32,
            t,
            z,
            0.8f32,
            1f32,
            if floor(modulo(
                9f32 * 1.2f32 + t * mix(1f32, 200f32, pow(hash(9f32, 9f32, 9f32) + 0.5f32, 2f32)),
                3f32
            )) > 0f32
            {
                -1f32
            } else {
                1f32
            }
        ) - 0.2f32,
        d,
    );
    let d = min(
        box3!(
            r - 2f32 * 10f32,
            t,
            z,
            0.8f32,
            1f32,
            if floor(modulo(
                10f32 * 1.2f32
                    + t * mix(1f32, 200f32, pow(hash(10f32, 10f32, 10f32) + 0.5f32, 2f32)),
                3f32
            )) > 0f32
            {
                -1f32
            } else {
                1f32
            }
        ) - 0.2f32,
        d,
    );
    let d = min(
        box3!(
            r - 2f32 * 11f32,
            t,
            z,
            0.8f32,
            1f32,
            if floor(modulo(
                11f32 * 1.2f32
                    + t * mix(1f32, 200f32, pow(hash(11f32, 11f32, 11f32) + 0.5f32, 2f32)),
                3f32
            )) > 0f32
            {
                -1f32
            } else {
                1f32
            }
        ) - 0.2f32,
        d,
    );
    let d = min(
        box3!(
            r - 2f32 * 12f32,
            t,
            z,
            0.8f32,
            1f32,
            if floor(modulo(
                12f32 * 1.2f32
                    + t * mix(1f32, 200f32, pow(hash(12f32, 12f32, 12f32) + 0.5f32, 2f32)),
                3f32
            )) > 0f32
            {
                -1f32
            } else {
                1f32
            }
        ) - 0.2f32,
        d,
    );
    let d = min(
        box3!(
            r - 2f32 * 13f32,
            t,
            z,
            0.8f32,
            1f32,
            if floor(modulo(
                13f32 * 1.2f32
                    + t * mix(1f32, 200f32, pow(hash(13f32, 13f32, 13f32) + 0.5f32, 2f32)),
                3f32
            )) > 0f32
            {
                -1f32
            } else {
                1f32
            }
        ) - 0.2f32,
        d,
    );
    let d = min(
        box3!(
            r - 2f32 * 14f32,
            t,
            z,
            0.8f32,
            1f32,
            if floor(modulo(
                14f32 * 1.2f32
                    + t * mix(1f32, 200f32, pow(hash(14f32, 14f32, 14f32) + 0.5f32, 2f32)),
                3f32
            )) > 0f32
            {
                -1f32
            } else {
                1f32
            }
        ) - 0.2f32,
        d,
    );
    let d = min(
        box3!(
            r - 2f32 * 15f32,
            t,
            z,
            0.8f32,
            1f32,
            if floor(modulo(
                15f32 * 1.2f32
                    + t * mix(1f32, 200f32, pow(hash(15f32, 15f32, 15f32) + 0.5f32, 2f32)),
                3f32
            )) > 0f32
            {
                -1f32
            } else {
                1f32
            }
        ) - 0.2f32,
        d,
    );
    let d = min(
        box3!(
            r - 2f32 * 16f32,
            t,
            z,
            0.8f32,
            1f32,
            if floor(modulo(
                16f32 * 1.2f32
                    + t * mix(1f32, 200f32, pow(hash(16f32, 16f32, 16f32) + 0.5f32, 2f32)),
                3f32
            )) > 0f32
            {
                -1f32
            } else {
                1f32
            }
        ) - 0.2f32,
        d,
    );
    let d = min(
        box3!(
            r - 2f32 * 17f32,
            t,
            z,
            0.8f32,
            1f32,
            if floor(modulo(
                17f32 * 1.2f32
                    + t * mix(1f32, 200f32, pow(hash(17f32, 17f32, 17f32) + 0.5f32, 2f32)),
                3f32
            )) > 0f32
            {
                -1f32
            } else {
                1f32
            }
        ) - 0.2f32,
        d,
    );
    let d = min(
        box3!(
            r - 2f32 * 18f32,
            t,
            z,
            0.8f32,
            1f32,
            if floor(modulo(
                18f32 * 1.2f32
                    + t * mix(1f32, 200f32, pow(hash(18f32, 18f32, 18f32) + 0.5f32, 2f32)),
                3f32
            )) > 0f32
            {
                -1f32
            } else {
                1f32
            }
        ) - 0.2f32,
        d,
    );
    let d = min(
        box3!(
            r - 2f32 * 19f32,
            t,
            z,
            0.8f32,
            1f32,
            if floor(modulo(
                19f32 * 1.2f32
                    + t * mix(1f32, 200f32, pow(hash(19f32, 19f32, 19f32) + 0.5f32, 2f32)),
                3f32
            )) > 0f32
            {
                -1f32
            } else {
                1f32
            }
        ) - 0.2f32,
        d,
    );
    let d = min(
        box3!(
            r - 2f32 * 20f32,
            t,
            z,
            0.8f32,
            1f32,
            if floor(modulo(
                20f32 * 1.2f32
                    + t * mix(1f32, 200f32, pow(hash(20f32, 20f32, 20f32) + 0.5f32, 2f32)),
                3f32
            )) > 0f32
            {
                -1f32
            } else {
                1f32
            }
        ) - 0.2f32,
        d,
    );
    min(d, -z + value_noise!(x, y, x, 0.1f32, 1f32) * 2f32)
}

pub fn temple(p: Vec3) -> f32 {
    let Vec3 { x, y, z } = p;
    let (a0, a1) = (0.1, 0.2);
    let d = 99f32;
    let [y, z] = rot0(y, z, a1);
    let f = y + abs(value_noise!(x, z, 1f32, 0f32, 3f32)) * 5f32;
    let [x, y, z] = [y, z, x];
    let [x, z] = rot0(x, z, a0);
    let [x, z] = rot0(x, z, a1);
    let x = smooth_abs(x, 2f32) - 3f32;
    let y = smooth_abs(y, 2f32) - 3f32;
    let z = smooth_abs(z, 2f32) - 3f32;
    let d = round_min(vec![d, torus(y, z, x, 5f32, 0.5f32 + 1f32 * 0.2f32), 1f32]);
    let [x, y, z] = [y, z, x];
    let [x, z] = rot0(x, z, a0);
    let [x, z] = rot0(x, z, a1);
    let x = smooth_abs(x, 2f32) - 3f32;
    let y = smooth_abs(y, 2f32) - 3f32;
    let z = smooth_abs(z, 2f32) - 3f32;
    let d = round_min(vec![d, torus(y, z, x, 5f32, 0.5f32 + 2f32 * 0.2f32), 1f32]);
    let [x, y, z] = [y, z, x];
    let [x, z] = rot0(x, z, a0);
    let [x, z] = rot0(x, z, a1);
    let x = smooth_abs(x, 2f32) - 3f32;
    let y = smooth_abs(y, 2f32) - 3f32;
    let z = smooth_abs(z, 2f32) - 3f32;
    let d = round_min(vec![d, torus(y, z, x, 5f32, 0.5f32 + 3f32 * 0.2f32), 1f32]);
    let [x, y, z] = [y, z, x];
    let [x, z] = rot0(x, z, a0);
    let [x, z] = rot0(x, z, a1);
    let x = smooth_abs(x, 2f32) - 3f32;
    let y = smooth_abs(y, 2f32) - 3f32;
    let z = smooth_abs(z, 2f32) - 3f32;
    let d = round_min(vec![d, torus(y, z, x, 5f32, 0.5f32 + 4f32 * 0.2f32), 1f32]);
    let [x, y, z] = [y, z, x];
    let [x, z] = rot0(x, z, a0);
    let [x, z] = rot0(x, z, a1);
    let x = smooth_abs(x, 2f32) - 3f32;
    let y = smooth_abs(y, 2f32) - 3f32;
    let z = smooth_abs(z, 2f32) - 3f32;
    let d = round_min(vec![d, torus(y, z, x, 5f32, 0.5f32 + 5f32 * 0.2f32), 1f32]);
    round_min(vec![
        f,
        length!(d, value_noise!(x, y, z, 0.5f32, 1f32)) - 0.1f32,
        0.5f32,
    ])
}

pub fn toy(p: Vec3) -> f32 {
    let Vec3 { x, y, z } = p;
    let (a0, a1) = (0.1, 0.2);
    let s = 1f32;
    let x1 = x - 0.25f32;
    let y1 = y - 4.6f32;
    let z1 = z;
    let x = x1;
    let y = y1;
    let [x, y] = rot0(x, y, a0);
    let [x, z] = rot0(x, z, a1);
    let x = smooth_abs(x * 2f32, 0.1f32) - 4f32;
    let y = smooth_abs(y * 2f32, 0.1f32) - 4f32;
    let z = smooth_abs(z * 2f32, 0.1f32) - 4f32;
    let s = s * 0.4f32;
    let [x, y] = rot0(x, y, a0);
    let [x, z] = rot0(x, z, a1);
    let x = smooth_abs(x * 2f32, 0.1f32) - 4f32;
    let y = smooth_abs(y * 2f32, 0.1f32) - 4f32;
    let z = smooth_abs(z * 2f32, 0.1f32) - 4f32;
    let s = s * 0.4f32;
    let [x, y] = rot0(x, y, a0);
    let [x, z] = rot0(x, z, a1);
    let x = smooth_abs(x * 2f32, 0.1f32) - 4f32;
    let y = smooth_abs(y * 2f32, 0.1f32) - 4f32;
    let z = smooth_abs(z * 2f32, 0.1f32) - 4f32;
    let s = s * 0.4f32;
    let [x, y] = rot0(x, y, a0);
    let [x, z] = rot0(x, z, a1);
    let x = smooth_abs(x * 2f32, 0.1f32) - 4f32;
    let y = smooth_abs(y * 2f32, 0.1f32) - 4f32;
    let z = smooth_abs(z * 2f32, 0.1f32) - 4f32;
    let s = s * 0.4f32;
    round_min(vec![
        round_min(vec![
            box3!(x, y, z, 4f32) * s - 0.01f32,
            round_max(vec![length!(x, y) - 1.75f32, box3!(x, y, z, 4f32) * 0.2f32]),
            0.75f32,
        ]),
        torus(x1, y1, z1, 9f32, 0.75f32),
        2.75f32,
    ])
}

pub fn ghost(p: Vec3) -> f32 {
    let Vec3 { x, y, z } = p;
    let (a0, a1) = (0.1, 0.3);
    let y = y - 9.8f32;
    let [x, z] = rot0(x, z, a0);
    let a = x;
    let b = y;
    let c = abs(z) - 0.3f32;
    let [a, b] = rot(a, b, cos(0.17f32), sin(0.17f32));
    let an = floor(0.5f32 + atan2(b, a) / a1) * a1;
    let [a, b] = rot(a, b, cos(an), sin(an));
    let d = intersect(vec![
        union(vec![
            box3!(a - 7f32, b, c, 0.01f32, 2f32, 0.01f32) - 0.05f32,
            box3!(b, a, c, 0.02f32, 7f32, 0.02f32) - 0.01f32,
            length!(a - 7f32, b) - 0.4f32,
            length!(modulo(clamp(a, 0f32, 5f32), 1f32) - 0.5f32, b) - 0.05f32,
        ]),
        abs(z) - 0.3f32,
    ]);
    let a = abs(x);
    let b = y;
    let an = 0.3f32;
    let [a, _] = rot(a, b, cos(an), sin(an));
    let d = union(vec![
        intersect(vec![
            union(vec![
                d,
                length!(x, y) - 0.2f32,
                length!(a, c - 0.3f32) - 0.1f32,
            ]),
            abs(z) - 0.7f32,
        ]),
        abs(y + 10f32) - 2f32 - sin(x * 0.1f32),
    ]);
    let t = 8f32 * floor(x / 8f32) + 4f32;
    let h = 20f32 - sin(t) * 10f32;
    union(vec![
        d,
        round_min(vec![
            intersect(vec![
                box3!(x - t, y + h * 0.5f32, z + 70f32, 3f32, h, 3f32),
                -box3!(
                    abs(x - t) - 1.5f32,
                    modulo(y, 3f32) - 1.5f32,
                    z + 68f32,
                    0.8f32,
                    h * 0.04f32,
                    2f32
                ),
            ]),
            length!(y + 9f32, z + 65f32) - 0.5f32 + value_noise!(x, y, z, 5f32, 1f32) * 10f32,
            2f32,
        ]),
    ])
}

pub fn shai_hulud(p: Vec3) -> f32 {
    let Vec3 { x, y, z } = p;
    let a0 = 0.1;
    let [x, z] = rot0(x, z, a0);
    let m = value_noise!(x, y, z, 0.4f32, 1f32);
    let r = 30f32;
    let t = 0.2f32;
    let v = atan2(z, y - 7f32);
    let u = length!(z, y - 7f32);
    min(
        min(
            max(
                torus(x + 5f32, y + 5.5f32, z + 10f32, 0.5f32, 0.05f32),
                y + 5.5f32,
            ),
            length!(x * 3f32 + 15f32, y / 4f32 + 2.4f32, z * 3f32 + 30f32) - 1f32,
        ),
        round_min(vec![
            y - 20f32
                + value_noise!(x / 3f32, y / 3f32, z / 2f32, 1f32, 1f32)
                + smoothstep(
                    30f32,
                    0f32,
                    15f32 - abs(x + sin(z / 2f32) + sin(z / 6f32) + sin(z / 8f32) - 4f32),
                ) * 15f32
                + smoothstep(
                    30f32,
                    0f32,
                    15f32
                        - abs(
                            modulo(z + x + sin(x / 2f32) + sin(x / 6f32) + sin(x / 8f32), 60f32)
                                - 35f32,
                        ),
                ) * 15f32,
            max(
                -length!(y - 7f32, z, x + 3f32) + 4f32 + sin(v * 3f32 - 2f32),
                min(
                    min(
                        torus(x - 90f32, y + 23f32, z, 20f32, 5f32 + m),
                        max(abs(torus(x, y + r - 7f32, z, r, 5f32 + m)) - t, -x),
                    ),
                    min(
                        max(abs(length!(x, y - 7f32, z) - 5f32 - m) - t, x),
                        length!(
                            u / 3f32 - 1f32,
                            abs(abs(abs(x * 2f32 - 4f32) - 2f32) - 2f32) - 1f32,
                            modulo(x / 2f32 + v * 9f32, 1.6f32) - 0.8f32
                        ) - 0.34f32,
                    ),
                ),
            ),
            2f32,
        ]),
    )
}

pub fn plato(p: Vec3) -> f32 {
    let Vec3 { x, y, z } = p;
    let (a0, a1) = (0.1, 0.2);
    let d = 99f32;
    let l = 10f32;
    let x = x - l * 2f32;
    let y = y - l;
    let z = z + 2.5f32;
    let x = x + l;
    let a = a0 * (1f32 + 2f32);
    let s = sin(a);
    let c = cos(a);
    let [x1, y1] = rot(x, y, s, c);
    let a = a1 * (1f32 + 2f32);
    let s = sin(a);
    let c = cos(a);
    let [x1, z1] = rot(x1, z, s, c);
    let d = round_min(vec![d, box3!(x1, y1, z1, 4f32), 3f32]);
    let x = x + l;
    let a = a0 * (2f32 + 2f32);
    let s = sin(a);
    let c = cos(a);
    let [x1, y1] = rot(x, y, s, c);
    let a = a1 * (2f32 + 2f32);
    let s = sin(a);
    let c = cos(a);
    let [x1, z1] = rot(x1, z, s, c);
    let d = round_min(vec![d, box3!(x1, y1, z1, 4f32), 3f32]);
    let x = x + l;
    let a = a0 * (3f32 + 2f32);
    let s = sin(a);
    let c = cos(a);
    let [x1, y1] = rot(x, y, s, c);
    let a = a1 * (3f32 + 2f32);
    let s = sin(a);
    let c = cos(a);
    let [x1, z1] = rot(x1, z, s, c);
    let d = round_min(vec![d, box3!(x1, y1, z1, 4f32), 3f32]);
    union(vec![
        d + 0.5f32,
        length!(
            value_noise!(x, y, z, 0.1f32, 1f32, 2f32) - 0.5f32,
            abs(d) - 0.1f32
        ) - 0.4f32,
    ])
}

pub fn pawns(p: Vec3) -> f32 {
    let Vec3 { x, y, z } = p;
    let i = modulo(floor(x / 8f32) + floor(z / 8f32), 2f32);
    let x = modulo(x, 8f32) - 4f32;
    let z = modulo(z, 8f32) - 4f32;
    let a = length!(x, y, z) - 1f32;
    let q = length!(x, z);
    let b = max(dot!(1f32, 0.3f32, q, y), -5f32 - y);
    let a = round_min(vec![a, b, 1f32]);
    let y = y + 1f32;
    let a = round_min(vec![a, length!(x, y * 5f32, z) - 0.8f32, 1f32]);
    let y = y + 3f32;
    let a = round_min(vec![a, length!(x, y * 2f32, z) - 1f32, 0.5f32]);
    let y = y + 1f32;
    let a = round_min(vec![a, length!(x, y * 3f32, z) - 1.7f32, 0.1f32]);
    min(a, y + 0.05f32 * i * value_noise!(x, y, z, 4f32, 0f32))
}

pub fn asurf(p: Vec3) -> f32 {
    let Vec3 { x, y, z } = p;
    let (a0, _) = (0.25, 0.2);
    let s = 1f32;
    let z = z + 1f32;
    let x = x + 0.7f32;
    let l = length!(x - 3f32, y - 1f32, z - 1f32) - 1.45f32;
    let x = x * 0.5f32;
    let y = y * 0.5f32;
    let z = z * 0.5f32;
    let yy = y;
    let x = poly_smooth_abs(x + 1f32, 0.1f32) - poly_smooth_abs(x - 1f32, 0.1f32) - x;
    let z = poly_smooth_abs(z + 1f32, 0.1f32) - poly_smooth_abs(z - 1f32, 0.1f32) - z;
    let y = y - 1f32;
    let x = x - 0.3f32;
    let [x, y] = rot0(x, y, a0);
    let sc = 2f32 / clamp(x * x + y * y + z * z, 0.4f32, 1f32);
    let x = x * sc;
    let y = y * sc;
    let z = z * sc;
    let s = s * sc;
    let x = poly_smooth_abs(x + 1f32, 0.1f32) - poly_smooth_abs(x - 1f32, 0.1f32) - x;
    let z = poly_smooth_abs(z + 1f32, 0.1f32) - poly_smooth_abs(z - 1f32, 0.1f32) - z;
    let y = y - 1f32;
    let x = x - 0.3f32;
    let [x, y] = rot0(x, y, a0);
    let sc = 2f32 / clamp(x * x + y * y + z * z, 0.4f32, 1f32);
    let x = x * sc;
    let y = y * sc;
    let z = z * sc;
    let s = s * sc;
    let x = poly_smooth_abs(x + 1f32, 0.1f32) - poly_smooth_abs(x - 1f32, 0.1f32) - x;
    let z = poly_smooth_abs(z + 1f32, 0.1f32) - poly_smooth_abs(z - 1f32, 0.1f32) - z;
    let y = y - 1f32;
    let x = x - 0.3f32;
    let [x, y] = rot0(x, y, a0);
    let sc = 2f32 / clamp(x * x + y * y + z * z, 0.4f32, 1f32);
    let x = x * sc;
    let y = y * sc;
    let z = z * sc;
    let s = s * sc;
    let x = poly_smooth_abs(x + 1f32, 0.1f32) - poly_smooth_abs(x - 1f32, 0.1f32) - x;
    let z = poly_smooth_abs(z + 1f32, 0.1f32) - poly_smooth_abs(z - 1f32, 0.1f32) - z;
    let y = y - 1f32;
    let x = x - 0.3f32;
    let [x, y] = rot0(x, y, a0);
    let sc = 2f32 / clamp(x * x + y * y + z * z, 0.4f32, 1f32);
    let x = x * sc;
    let y = y * sc;
    let s = s * sc;
    round_min(vec![
        l,
        union(vec![yy, (length!(x, y) - 1.5f32) / s * 2f32]),
        0.2f32,
    ])
}
