use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::AddAssign;
use std::ops::SubAssign;
use std::ops::MulAssign;
use std::ops::DivAssign;
use std::marker::Sized;
use crate::core::scalar::Scalar;

pub trait Vector
    : Clone
    + Copy
    + Sized
    + Add<Vector2, Output = Self>
    + Sub<Vector2, Output = Self>
    + Mul<Scalar, Output = Self>
    + Div<Scalar, Output = Self>
    + AddAssign<Vector2>
    + SubAssign<Vector2>
    + MulAssign<Scalar>
    + DivAssign<Scalar>
{
    fn zero() -> Self;
    fn dot(self, rhs: Self) -> Scalar;
    fn length_squared(self) -> Scalar {
        self.dot(self)
    }
    fn length(self) -> Scalar {
        self.length_squared().sqrt()
    }
    fn longer_than(self, rhs: Self) -> bool {
        self.length_squared() > rhs.length_squared()
    }
    fn normalize(self) -> Self {
        self / self.length()
    }
    fn normalize_to(self, length: Scalar) -> Self {
        self.normalize()
    }
}


#[derive(Clone, Copy)]
pub struct Vector2 {
    x: Scalar,
    y: Scalar,
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
