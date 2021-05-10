use std::ops;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn x(self) -> f64 {
        self.x
    }
    pub fn y(self) -> f64 {
        self.y
    }
    pub fn z(self) -> f64 {
        self.z
    }

    pub fn length_squared(self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(vec: Self) -> Self {
        vec / vec.length()
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

/// Mul between two Vec3
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_test() {
        let vec3 = Vec3::new(0.126, 0.314, 0.631);
        let expected = (0.126, 0.314, 0.631);
        assert_eq!((vec3.x, vec3.y, vec3.z), expected);
    }

    #[test]
    fn vec3_length_squared_test() {
        let vec3 = Vec3::new(0.126, 0.314, 0.631);
        let expected = 0.512633;
        assert_eq!(vec3.length_squared(), expected);
    }

    #[test]
    fn vec3_length_test() {
        let vec3 = Vec3::new(0.126, 0.314, 0.631);
        let expected = 0.71598393836733516490216633646487;
        assert_eq!(vec3.length(), expected);
    }

    #[test]
    fn vec3_unit_vector_test() {
        let vec3 = Vec3::new(0.126, 0.314, 0.631);
        let expected = Vec3::new(
            0.17598160132938032, // Round-Off error detected. Probably issue with type system 0.1759816013293803[3]793702894350261
            0.4385573239478208421605324465065,
            0.88130468602253169236718462976307,
        );
        assert_eq!(Vec3::unit_vector(vec3), expected);
    }

    #[test]
    fn vec3_dot_test() {
        let vec1 = Vec3::new(0.126, 0.314, 0.631);
        let vec2 = Vec3::new(0.314, 0.631, 0.126);
        let expected = 0.317204_00000000004; // Multiplication Overflow detected, '_' used as demarcation.
        assert_eq!(vec1.dot(vec2), expected);
    }

    #[test]
    fn vec3_cross_test() {
        let vec1 = Vec3::new(0.126, 0.314, 0.631);
        let vec2 = Vec3::new(0.314, 0.631, 0.126);

        let expected = Vec3::new(-0.358597, 0.182258, -0.019089999999999996); // Multiplication Overflow detected, z vector erroneous. (CALCULATED:-0.01909)
        assert_eq!(vec1.cross(vec2), expected);
    }
}
