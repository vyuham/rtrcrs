use nalgebra::Vector3;

use crate::{HitRecord, Hittable, Point3, Ray};

/// Defines a geometrically Spherical object.
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    /// Provides a definition of hit() for spherical objects.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let (a, half_b, c) = (
            ray.direction().dot(&ray.direction()),
            oc.dot(&ray.direction()),
            oc.dot(&oc) - self.radius.powi(2),
        );

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lied in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);

        let mut rec = HitRecord {
            point,
            normal: Vector3::new(0.0, 0.0, 0.0),
            t,
            front_face: false,
        };

        let outward_normal = (point - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);

        Some(rec)
    }
}
