use crate::core::{MaterialFn, Sdf, Surface};
use glam::Vec3Swizzles;
use glam::{Affine3A, Vec2, Vec3};

pub fn sd_sphere(radius: f32, center: Vec3, material: MaterialFn) -> Sdf {
    Box::new(move |p| Surface::new((p - center).length() - radius, material.clone()))
}

pub fn sd_box(b: Vec3, transform: Affine3A, material: MaterialFn) -> Sdf {
    Box::new(move |p| {
        let p = transform.transform_point3(p);
        let q = p.abs() - b;
        Surface::new(
            q.y.max(q.z).max(q.x).min(0.0) + q.max(Vec3::ZERO).length(),
            material.clone(),
        )
    })
}

pub fn sd_round_box(b: Vec3, radius: f32, transform: Affine3A, material: MaterialFn) -> Sdf {
    Box::new(move |p| {
        let p = transform.transform_point3(p);
        let q = p.abs() - b;
        Surface::new(
            q.x.max(q.y).max(q.z).min(0.0) + q.max(Vec3::ZERO).length() - radius,
            material.clone(),
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
        let q = Vec2::new(p.xz().length() - radius1, p.y);
        Surface::new(q.length() - radius2, material.clone())
    })
}