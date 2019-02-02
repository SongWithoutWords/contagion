use crate::core::scalar::Scalar;
use crate::core::vector::*;

struct Mat4 {
    x: Vector4,
    y: Vector4,
    z: Vector4,
    w: Vector4
}

pub fn multiply_vec2(vec: Vector2, matrix: Mat4) -> Vector2 {
    let vec4:Vector4 = Vector4 { x: vec.x, y: vec.y, z: 0.0, w: 1.0 };
    let vec


    return vec2;
}