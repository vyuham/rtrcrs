use crate::{
    definitions::{degrees_to_radians, random_in_unit_sphere},
    Point3, Ray, Vec3,
};
/// Defines a data-structure used to store the geometry of the Camera.
#[derive(Clone, Copy)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Point3,
    vertical: Point3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    /// Used to set the geometric values of the Camera.
    pub fn new(
        position: &Point3,
        focus: &Point3,
        vup: &Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (position - focus).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);
        let lens_radius = aperture / 2.0;
        let origin = *position;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius,
        }
    }
    /// Used to get the Ray corresponding to a Pixel and the Camera.
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_sphere();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ray_test() {
        let vec = &Vec3::new(1.0, 1.0, 1.0);
        let cam = Camera::new(vec, vec, vec, 1.0, 1.0, 1.0, 1.0);
        let nan_vec = Vec3::new(f64::NAN, f64::NAN, f64::NAN);
        let expected = format!("{:?}", Ray::new(nan_vec, nan_vec));
        let ray = format!("{:?}", cam.get_ray(1.0, 1.0));

        assert_eq!(ray, expected);
    }
}