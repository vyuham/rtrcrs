use std::ops;

/// Defines a three dimensional data structure that can be used for purposes such as represeting a point in the 3D Cartesian Co-ordinate System or per-pixel color values.
/// Operator overloading is used for operations with objects of Vec3, by implementing the traits from [std::ops](std::ops).
#[derive(Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    /// Creates a new Vec3.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// This function returns the x co-ordintate of given vector.
    pub fn x(self) -> f64 {
        self.x
    }
        
    /// This function returns the y co-ordintate of given vector.
    pub fn y(self) -> f64 {
        self.y
    }

    /// This function returns the z co-ordintate of given vector.
    pub fn z(self) -> f64 {
        self.z
    }

    /// This function returns the dot product of given vector with any other vector.
    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// This function returns the cross product of given vector with any other vector.
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    /// This fucntion returns the square of given vector.
    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }

    /// This fucntion returns the length of given vector.
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    /// This fucntion produces the unit vector of any input vector.
    pub fn unit_vector(vec: Self) -> Self {
        vec / vec.length()
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

/// Mul between two Vec3
impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}
