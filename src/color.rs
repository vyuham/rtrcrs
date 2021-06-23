/// Defines a pixel's color as an RGB value.
pub type Color = crate::Vec3;

/// Used to smoothen edges using Antialiasing.
pub fn anti_aliased(color: Color, samples_per_pixel: i32) -> Color {
    let sample = |s: f64| (s / samples_per_pixel as f64).sqrt().max(0.0).min(0.999);

    Color::new(sample(color.x), sample(color.y), sample(color.z))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anti_aliasing_test() {
        let c = Color::new(0.9, 0.2, 0.1);
        let aac = anti_aliased(c, 2);

        assert_eq!(aac, Color::new(0.6708203932499369, 0.31622776601683794, 0.22360679774997896));
    }
}