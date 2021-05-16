use std::fmt;

/// Defines a pixel's color as an RGB value.
pub type Color = crate::Vec3;

impl Color {
    /// Used to smoothen edges using Antialiasing.
    pub fn anti_aliased(&self, samples_per_pixel: i32) -> Self {
        let sample = |s: f64| (s / samples_per_pixel as f64).max(0.0).min(0.999);

        Self::new(sample(self.x()), sample(self.y()), sample(self.z()))
    }
}

impl fmt::Display for Color {
    /// Write the translated \[0,255\] value of each Color.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            (256.0 * self.x()) as u8,
            (256.0 * self.y()) as u8,
            (256.0 * self.z()) as u8,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_string_test() {
        let color = Color::new(0.126, 0.314, 0.631);
        let expected = "32 80 161".to_string();
        assert_eq!(format!("{}", color), expected);
    }
}
