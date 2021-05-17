use nalgebra::Vector3;

use crate::{
    definitions::random_in_hemisphere, Color, Hittable, HittableList, INFINITY,
};

/// Defines an alias for Vector3, used to define a point in 3-dimensional co-ordinate space.
pub type Point3 = Vector3<f64>;

/// Defines a Ray using a reference starting point and a direction vector.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Ray {
    orig: Point3,
    dir: Vector3<f64>,
}

impl Ray {
    /// This function creates a new Ray.
    pub fn new(orig: Point3, dir: Vector3<f64>) -> Self {
        Self { orig, dir }
    }

    /// Returns the origin of the given Ray.
    pub fn origin(&self) -> Point3 {
        self.orig
    }

    /// Returns the direction of the given Ray.
    pub fn direction(&self) -> Vector3<f64> {
        self.dir
    }

    /// Returns point along Ray at `t`.
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }

    /// Returns the expected color at the intersection of any ray and the object(s) in `world`.
    pub fn color(&self, world: &HittableList, depth: i32) -> Color {
        if depth <= 0 {
            Color::new(0.0, 0.0, 0.0)
        } else if let Some(rec) = world.hit(self, 0.001, INFINITY) {
            let mut scattered=Ray::new(Point3::new(0.0, 0.0, 0.0),  Vector3::new(0.0, 0.0, 0.0));
            let mut attenuation = Color::new(0.0, 0.0, 0.0);
            if rec.material.scatter(self, &rec, &mut attenuation, &mut scattered){
                attenuation.zip_map(&scattered.color(world, depth-1), |l,r| l*r)
            } else {
                Color::new(0.0, 0.0, 0.0)
            }
        } else {
            let unit_dir = self.dir.normalize();
            let t = 0.5 * (unit_dir.y + 1.0);
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
        let dir = Vector3::new(2.0, 3.0, 5.0);
        let ray = Ray::new(origin, dir);
        
        assert_eq!(ray.origin(), origin);
        assert_eq!(ray.direction(), dir);
        assert_eq!(ray.at(3.0), origin + dir * 3.0);
    }
    #[test]
    fn color_test() {
        let origin = Point3::new(3.0, 2.0, 1.0);
        let dir = Vector3::new(2.0, 3.0, 5.0);
        let world = HittableList::default();
        let color = Ray::new(origin, dir).color(&world, 10);

        assert_eq!(
            color,
            Color::new(0.6283339341519281, 0.7770003604911568, 1.0)
        );
    }
}
