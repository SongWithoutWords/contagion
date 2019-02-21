use std::ops::Neg;
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
    + Neg<Output = Self>
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Scalar, Output = Self>
    + Div<Scalar, Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
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
        self.normalize() * length
    }
}



