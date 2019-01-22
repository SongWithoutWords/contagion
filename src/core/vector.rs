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

// Vector 2

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

// Vector 3

#[derive(Clone, Copy)]
struct Vector3 {
    x: Scalar,
    y: Scalar,
    z: Scalar,
}

impl Add for Vector3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vector3 {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vector3 {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl Mul<Scalar> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: Scalar) -> Self {
        Vector3 {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

impl Div<Scalar> for Vector3 {
    type Output = Self;
    fn div(self, rhs: Scalar) -> Self {
        Vector3 {x: self.x / rhs, y: self.y / rhs, z: self.z / rhs}
    }
}

impl Vector for Vector3  {
    fn zero() -> Self {
        Vector3{x: 0 as Scalar, y: 0 as Scalar, z: 0 as Scalar}
    }
    fn dot(self, rhs: Self) -> Scalar {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}