use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::marker::Sized;
use crate::core::scalar::Scalar;

trait Vector : Clone + Copy + Sized + Add + Sub + Mul<Scalar> + Div<Scalar> {
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
}

#[derive(Clone, Copy)]
struct Vector2 {
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

impl Div<Scalar> for Vector2 {
    type Output = Self;
    fn div(self, rhs: Scalar) -> Self {
        Vector2 {x: self.x / rhs, y: self.y / rhs}
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
