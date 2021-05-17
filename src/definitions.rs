use nalgebra::Vector3;
use rand::Rng;

/// Re-exports the definition of the constant PI.
pub use std::f64::consts::PI;

/// Re-exports the definition of the constant MAX as INFINITY.
pub const INFINITY: f64 = std::f64::MAX;

/// Provides logic for converting Degrees to Radians.
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// Provides logic for generating random numbers of type f64 from 0.0 to 1.0 .
pub fn random_double(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

pub fn random_vec(min: f64, max: f64) -> Vector3<f64> {
    let rd = || random_double(min, max);
    Vector3::new(rd(), rd(), rd())
}

pub fn random_in_unit_sphere() -> Vector3<f64> {
    loop {
        let p = random_vec(-1.0, 1.0);
        if p.dot(&p) >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_in_hemisphere(normal: &Vector3<f64>) -> Vector3<f64> {
    let in_unit_sphere = random_in_unit_sphere();
    // In the same hemisphere as the normal
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn random_unit_vector() -> Vector3<f64> {
    random_in_unit_sphere().normalize()
}

pub fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * v.dot(n)*n
}

pub fn near_zero(v:Vector3<f64>) -> bool{
    let s = 10f64.powi(-8);
    v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
}