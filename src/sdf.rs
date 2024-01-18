use crate::core::{MaterialFn, Sdf, Surface};
use glam::Vec3Swizzles;
use glam::{Affine3A, Vec2, Vec3};

pub fn sd_plane(normal: Vec3, transform: Affine3A, material: MaterialFn) -> Sdf {
    Box::new(move |p| {
        let p = transform.transform_point3(p);
        let m = material(p);
        Surface::new(normal.dot(p) + 1.0, m)
    })
}

pub fn sd_sphere(radius: f32, center: Vec3, transform: Affine3A, material: MaterialFn) -> Sdf {
    Box::new(move |p| {
        let m = material(p);
        Surface::new(transform.transform_point3(p - center).length() - radius, m)
    })
}

pub fn sd_box(b: Vec3, center: Vec3, transform: Affine3A, material: MaterialFn) -> Sdf {
    Box::new(move |p| {
        let p = transform.transform_point3(p - center);
        let m = material(p);
        let q = p.abs() - b;
        Surface::new(
            q.y.max(q.z).max(q.x).min(0.0) + q.max(Vec3::ZERO).length(),
            m,
        )
    })
}

pub fn sd_round_box(
    b: Vec3,
    radius: f32,
    center: Vec3,
    transform: Affine3A,
    material: MaterialFn,
) -> Sdf {
    Box::new(move |p| {
        let p = transform.transform_point3(p - center);
        let m = material(p);
        let q = p.abs() - b;
        Surface::new(
            q.x.max(q.y).max(q.z).min(0.0) + q.max(Vec3::ZERO).length() - radius,
            m,
        )
    })
}

pub fn sd_torus(
    radius1: f32,
    radius2: f32,
    center: Vec3,
    transform: Affine3A,
    material: MaterialFn,
) -> Sdf {
    Box::new(move |p| {
        let p = transform.transform_point3(p - center);
        let m = material(p);
        let q = Vec2::new(p.xz().length() - radius1, p.y);
        Surface::new(q.length() - radius2, m)
    })
}

pub fn sd_capsule(
    radius: f32,
    center: Vec3,
    a: Vec3,
    b: Vec3,
    transform: Affine3A,
    material: MaterialFn,
) -> Sdf {
    Box::new(move |p| {
        let p = transform.transform_point3(p - center);
        let m = material(p);
        let pa = p - a;
        let ba = b - a;
        let h = (pa.dot(ba) / ba.dot(ba)).clamp(0.0, 1.0);
        Surface::new((pa - ba * h).length() - radius, m)
    })
}

pub fn sd_cylinder(
    radius: f32,
    center: Vec3,
    bottom: Vec3,
    top: Vec3,
    transform: Affine3A,
    material: MaterialFn,
) -> Sdf {
    Box::new(move |p| {
        let p = transform.transform_point3(p - center);
        let m = material(p);
        let pa = p - bottom;
        let ba = top - bottom;
        let baba = ba.dot(ba);
        let paba = pa.dot(ba);
        let x = (pa * baba - ba * paba).length() - radius * baba;
        let y = (paba - baba * 0.5).abs() - baba * 0.5;
        let x2 = x * x;
        let y2 = y * y * baba;
        let d = if x.max(y) < 0.0 {
            -x2.min(y2)
        } else {
            (if x > 0.0 { x2 } else { 0.0 }) + (if y > 0.0 { y2 } else { 0.0 })
        };
        let d = d.signum() * d.abs().sqrt() / baba;
        Surface::new(d, m)
    })
}
