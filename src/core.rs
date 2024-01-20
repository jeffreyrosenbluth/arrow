use glam::{Affine3A, Vec3};

pub const I: Affine3A = Affine3A::IDENTITY;

pub fn v3(value: f32) -> Vec3 {
    Vec3::new(value, value, value)
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

#[derive(Clone, Copy)]
pub struct Material {
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Material {
    pub fn color(c: Vec3, shininess: f32) -> Self {
        Self {
            ambient: 0.5 * grayscale(c),
            diffuse: 0.5 * grayscale(c),
            specular: 0.5,
            shininess,
        }
    }
}

pub type MaterialFn = fn(Vec3) -> Material;

pub fn grayscale(color: Vec3) -> f32 {
    0.2989 * color[0] + 0.5870 * color[1] + 0.1140 * color[2]
}

pub struct Surface {
    pub sd: f32,
    pub material: Material,
}

impl Surface {
    pub fn new(sd: f32, material: Material) -> Self {
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

pub type Sdf = Box<dyn Fn(Vec3) -> Surface + Sync>;

pub fn union(sdf1: Sdf, sdf2: Sdf) -> Sdf {
    Box::new(move |p| sdf1(p).union(sdf2(p)))
}

pub fn intersect(sdf1: Sdf, sdf2: Sdf) -> Sdf {
    Box::new(move |p| sdf1(p).intersect(sdf2(p)))
}

pub fn difference(sdf1: Sdf, sdf2: Sdf) -> Sdf {
    Box::new(move |p| sdf1(p).difference(sdf2(p)))
}

pub fn perturb(sdf: Sdf, f: fn(Vec3) -> f32) -> Sdf {
    Box::new(move |p| {
        let Surface { sd, material } = sdf(p);
        Surface::new(sd + f(p), material)
    })
}

pub fn round(sdf: Sdf, radius: f32) -> Sdf {
    Box::new(move |p| {
        let Surface { sd, material } = sdf(p);
        Surface::new(sd - radius, material)
    })
}

pub fn unions(sdfs: Vec<Sdf>) -> Sdf {
    sdfs.into_iter().reduce(|acc, sdf| union(acc, sdf)).unwrap()
}

pub fn intersects(sdfs: Vec<Sdf>) -> Sdf {
    sdfs.into_iter()
        .reduce(|acc, sdf| intersect(acc, sdf))
        .unwrap()
}
