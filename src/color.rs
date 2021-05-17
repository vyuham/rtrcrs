/// Defines a pixel's color as an RGB value.
pub type Color = nalgebra::Vector3<f64>;

/// Used to smoothen edges using Antialiasing.
pub fn anti_aliased(color: Color, samples_per_pixel: i32) -> Color {
    let sample = |s: f64| (s / samples_per_pixel as f64).sqrt().max(0.0).min(0.999);

    Color::new(sample(color.x), sample(color.y), sample(color.z))
}
