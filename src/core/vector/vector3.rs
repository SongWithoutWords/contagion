use crate::core::scalar::Scalar;
use crate::core::vector::Vector;

use std::ops::*;

#[derive(Clone, Copy, Debug)]
pub struct Vector3 {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
}

impl Add for Vector3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<Scalar> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: Scalar) -> Self {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

// Unfortunately not possible to do this more generally for all vectors, see link:
// https://users.rust-lang.org/t/implementing-generic-trait-with-local-struct-on-local-trait/23225
impl Mul<Vector3> for Scalar {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        rhs * self
    }
}


impl Div<Scalar> for Vector3 {
    type Output = Self;
    fn div(self, rhs: Scalar) -> Self {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self){
        *self = *self + rhs;
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign<Scalar> for Vector3 {
    fn mul_assign(&mut self, rhs: Scalar) {
        *self = *self * rhs;
    }
}

impl DivAssign<Scalar> for Vector3 {
    fn div_assign(&mut self, rhs: Scalar) {
        *self = *self / rhs;
    }
}

impl Vector for Vector3  {
    fn zero() -> Self {
        Vector3 {
            x: 0 as Scalar,
            y: 0 as Scalar,
            z: 0 as Scalar,
        }
    }
    fn dot(self, rhs: Self) -> Scalar {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
