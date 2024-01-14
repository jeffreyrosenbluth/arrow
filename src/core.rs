use glam::Vec3;

pub fn v3(value: f32) -> Vec3 {
    Vec3::new(value, value, value)
}

#[derive(Clone, Copy)]
pub struct Material {
    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
    pub shininess: f32,
}

impl Material {
    pub fn color(c: Vec3, shininess: f32) -> Self {
        Self {
            ambient: 0.5 * c,
            diffuse: 0.5 * c,
            specular: 0.5 * v3(1.0),
            shininess,
        }
    }
}

pub type MaterialFn = fn(Vec3) -> Material;

pub struct Surface {
    pub sd: f32,
    pub material: MaterialFn,
}

impl Surface {
    pub fn new(sd: f32, material: MaterialFn) -> Self {
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

pub type Sdf = Box<dyn Fn(Vec3) -> Surface>;

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