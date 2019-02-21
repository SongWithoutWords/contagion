use crate::core::scalar::Scalar;
use crate::core::vector::Vector;

use std::ops::*;

#[derive(Clone, Copy, Debug)]
pub struct Vector4 {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
    pub w: Scalar,
}

pub fn vector4(x: Scalar, y: Scalar, z: Scalar, w: Scalar) -> Vector4 {
    Vector4{ x: x, y: y, z: z, w: w }
}


impl Neg for Vector4 {
    type Output = Self;
    fn neg(self) -> Self {
        vector4(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Add for Vector4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vector4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Vector4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vector4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Mul<Scalar> for Vector4 {
    type Output = Self;
    fn mul(self, rhs: Scalar) -> Self {
        Vector4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

// Unfortunately not possible to do this more generally for all vectors, see link:
// https://users.rust-lang.org/t/implementing-generic-trait-with-local-struct-on-local-trait/23225
impl Mul<Vector4> for Scalar {
    type Output = Vector4;
    fn mul(self, rhs: Vector4) -> Vector4 {
        rhs * self
    }
}


impl Div<Scalar> for Vector4 {
    type Output = Self;
    fn div(self, rhs: Scalar) -> Self {
        Vector4 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl AddAssign for Vector4 {
    fn add_assign(&mut self, rhs: Self){
        *self = *self + rhs;
    }
}

impl SubAssign for Vector4 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign<Scalar> for Vector4 {
    fn mul_assign(&mut self, rhs: Scalar) {
        *self = *self * rhs;
    }
}

impl DivAssign<Scalar> for Vector4 {
    fn div_assign(&mut self, rhs: Scalar) {
        *self = *self / rhs;
    }
}

impl Vector for Vector4  {
    fn zero() -> Self {
        Vector4 {
            x: 0 as Scalar,
            y: 0 as Scalar,
            z: 0 as Scalar,
            w: 0 as Scalar,
        }
    }
    fn dot(self, rhs: Self) -> Scalar {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}
