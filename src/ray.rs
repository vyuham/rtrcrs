use crate::{Color, Vec3};
pub type Point3 = Vec3;

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    pub fn origin(self) -> Point3 {
        self.orig
    }
    pub fn direction(self) -> Vec3 {
        self.dir
    }
    pub fn at(self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }

    pub fn color(self) -> Color {
        let unit_dir = Vec3::unit_vector(self.dir);
        let t = 0.5 * (unit_dir.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
