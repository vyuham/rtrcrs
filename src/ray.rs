use crate::{Color, HitRecord, Hittable, HittableList, Vec3, INFINITY};

/// Defines an alias for Vec3, used to define a point in 3-dimensional co-ordinate space.
pub type Point3 = Vec3;

/// Defines a Ray using a reference starting point and a direction vector.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    /// This function creates a new Ray.
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    /// Returns the origin of the given Ray.
    pub fn origin(&self) -> Point3 {
        self.orig
    }

    /// Returns the direction of the given Ray.
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    /// Returns point along Ray at `t`.
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }

    /// Returns the expected color at the intersection of any ray and the object(s) in `world`.
    pub fn color(&self, world: &HittableList) -> Color {
        let mut rec = HitRecord::default();
        if world.hit(self, 0.0, INFINITY, &mut rec) {
            0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0))
        } else {
            let unit_dir = Vec3::unit_vector(self.dir);
            let t = 0.5 * (unit_dir.y() + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_test() {
        let origin = Point3::new(3.0, 2.0, 1.0);
        let dir = Vec3::new(2.0, 3.0, 5.0);
        let ray = Ray::new(origin, dir);
        assert_eq!(ray.origin(), origin);
        assert_eq!(ray.direction(), dir);
        assert_eq!(ray.at(3.0), origin + dir * 3.0);
    }
    #[test]
    fn color_test() {
        let origin = Point3::new(3.0, 2.0, 1.0);
        let dir = Vec3::new(2.0, 3.0, 5.0);
        let color = Ray::new(origin, dir).color();
        assert_eq!(color, Color::new(0.6283339341519281, 0.7770003604911568, 1.0));
    }
}
