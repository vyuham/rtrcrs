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
pub fn random_double() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}
