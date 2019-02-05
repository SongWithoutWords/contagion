use crate::core::scalar::Scalar;
use crate::core::vector::Vector;

use std::ops::*;

#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
    pub x: Scalar,
    pub y: Scalar,
}

impl Vector2 {
    pub fn from_angle(angle: Scalar) -> Vector2 {
        let (sin, cos) = angle.sin_cos();
        vector2(cos, sin)
    }
    pub fn angle(&self) -> Scalar {
        self.y.atan2(self.x)
    }
    pub fn as_f32_array(&self) -> [f32; 2] {
        [self.x as f32, self.y as f32]
    }
}

pub fn vector2(x: Scalar, y: Scalar) -> Vector2 {
    Vector2{ x: x, y: y }
}

impl Neg for Vector2 {
    type Output = Vector2;
    fn neg(self) -> Self {
        vector2(-self.x, -self.y)
    }
}

impl Add for Vector2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vector2 {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl Sub for Vector2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vector2 {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl Mul<Scalar> for Vector2 {
    type Output = Self;
    fn mul(self, rhs: Scalar) -> Self {
        Vector2 {x: self.x * rhs, y: self.y * rhs}
    }
}

// Unfortunately not possible to do this more generally for all vectors, see link:
// https://users.rust-lang.org/t/implementing-generic-trait-with-local-struct-on-local-trait/23225
impl Mul<Vector2> for Scalar {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Vector2 {
        rhs * self
    }
}


impl Div<Scalar> for Vector2 {
    type Output = Self;
    fn div(self, rhs: Scalar) -> Self {
        Vector2 {x: self.x / rhs, y: self.y / rhs}
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self){
        *self = *self + rhs;
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign<Scalar> for Vector2 {
    fn mul_assign(&mut self, rhs: Scalar) {
        *self = *self * rhs;
    }
}

impl DivAssign<Scalar> for Vector2 {
    fn div_assign(&mut self, rhs: Scalar) {
        *self = *self / rhs;
    }
}

impl Vector for Vector2  {
    fn zero() -> Self {
        Vector2{x: 0 as Scalar, y: 0 as Scalar}
    }
    fn dot(self, rhs: Self) -> Scalar {
        self.x * rhs.x + self.y * rhs.y
    }
}
