use crate::core::scalar::Scalar;
use crate::core::vector::*;

#[derive(Clone, Copy, Debug)]
pub struct Mat4 {
    pub i: Vector4,
    pub j: Vector4,
    pub k: Vector4,
    pub w: Vector4
}

pub fn multiply_vec2(vec: Vector2, matrix: Mat4) -> Vector2 {
    let vec4:Vector4 = Vector4 { x: vec.x, y: vec.y, z: 0.0, w: 1.0 };

    let row1:Vector4 = Vector4 { x: matrix.i.x, y: matrix.j.x, z: matrix.k.x, w: matrix.w.x };
    let row2:Vector4 = Vector4 { x: matrix.i.y, y: matrix.j.y, z: matrix.k.y, w: matrix.w.y };

    let new_x = row1.dot(vec4);
    let new_y = row2.dot(vec4);

    return Vector2 { x: new_x, y: new_y };
}

pub fn inverse_matrix4(matrix: Mat4) -> Mat4 {
    let a2323 = matrix.k.z * matrix.w.w - matrix.k.w * matrix.w.z;
    let a1323 = matrix.k.y * matrix.w.w - matrix.k.w * matrix.w.y;
    let a1223 = matrix.k.y * matrix.w.z - matrix.k.z * matrix.w.y;
    let a0323 = matrix.k.x * matrix.w.w - matrix.k.w * matrix.w.x;
    let a0223 = matrix.k.x * matrix.w.z - matrix.k.z * matrix.w.x;
    let a0123 = matrix.k.x * matrix.w.y - matrix.k.y * matrix.w.x;
    let a2313 = matrix.j.z * matrix.w.w - matrix.j.w * matrix.w.z;
    let a1313 = matrix.j.y * matrix.w.w - matrix.j.w * matrix.w.y;
    let a1213 = matrix.j.y * matrix.w.z - matrix.j.z * matrix.w.y;
    let a2312 = matrix.j.z * matrix.k.w - matrix.j.w * matrix.k.z;
    let a1312 = matrix.j.y * matrix.k.w - matrix.j.w * matrix.k.y;
    let a1212 = matrix.j.y * matrix.k.z - matrix.j.z * matrix.k.y;
    let a0313 = matrix.j.x * matrix.w.w - matrix.j.w * matrix.w.x;
    let a0213 = matrix.j.x * matrix.w.z - matrix.j.z * matrix.w.x;
    let a0312 = matrix.j.x * matrix.k.w - matrix.j.w * matrix.k.x;
    let a0212 = matrix.j.x * matrix.k.z - matrix.j.z * matrix.k.x;
    let a0113 = matrix.j.x * matrix.w.y - matrix.j.y * matrix.w.x;
    let a0112 = matrix.j.x * matrix.k.y - matrix.j.y * matrix.k.x;

    let det:f64 = ( matrix.i.x * ( matrix.j.y * a2323 - matrix.j.z * a1323 + matrix.j.w * a1223 )
        - matrix.i.y * ( matrix.j.x * a2323 - matrix.j.z * a0323 + matrix.j.w * a0223 )
        + matrix.i.z * ( matrix.j.x * a1323 - matrix.j.y * a0323 + matrix.j.w * a0123 )
        - matrix.i.w * ( matrix.j.x * a1223 - matrix.j.y * a0223 + matrix.j.z * a0123 ));

    let ix = 1.0 / det *   ( matrix.j.y * a2323 - matrix.j.z * a1323 + matrix.j.w * a1223 );
    let iy = 1.0 / det * - ( matrix.i.y * a2323 - matrix.i.z * a1323 + matrix.i.w * a1223 );
    let iz = 1.0 / det *   ( matrix.i.y * a2313 - matrix.i.z * a1313 + matrix.i.w * a1213 );
    let iw = 1.0 / det * - ( matrix.i.y * a2312 - matrix.i.z * a1312 + matrix.i.w * a1212 );
    let jx = 1.0 / det * - ( matrix.j.x * a2323 - matrix.j.z * a0323 + matrix.j.w * a0223 );
    let jy = 1.0 / det *   ( matrix.i.x * a2323 - matrix.i.z * a0323 + matrix.i.w * a0223 );
    let jz = 1.0 / det * - ( matrix.i.x * a2313 - matrix.i.z * a0313 + matrix.i.w * a0213 );
    let jw = 1.0 / det *   ( matrix.i.x * a2312 - matrix.i.z * a0312 + matrix.i.w * a0212 );
    let kx = 1.0 / det *   ( matrix.j.x * a1323 - matrix.j.y * a0323 + matrix.j.w * a0123 );
    let ky = 1.0 / det * - ( matrix.i.x * a1323 - matrix.i.y * a0323 + matrix.i.w * a0123 );
    let kz = 1.0 / det *   ( matrix.i.x * a1313 - matrix.i.y * a0313 + matrix.i.w * a0113 );
    let kw = 1.0 / det * - ( matrix.i.x * a1312 - matrix.i.y * a0312 + matrix.i.w * a0112 );
    let wx = 1.0 / det * - ( matrix.j.x * a1223 - matrix.j.y * a0223 + matrix.j.z * a0123 );
    let wy = 1.0 / det *   ( matrix.i.x * a1223 - matrix.i.y * a0223 + matrix.i.z * a0123 );
    let wz = 1.0 / det * - ( matrix.i.x * a1213 - matrix.i.y * a0213 + matrix.i.z * a0113 );
    let ww = 1.0 / det *   ( matrix.i.x * a1212 - matrix.i.y * a0212 + matrix.i.z * a0112 );

    let i:Vector4 = Vector4 {x: ix, y: iy, z: iz, w: iw};
    let j:Vector4 = Vector4 {x: jx, y: jy, z: jz, w: jw};
    let k:Vector4 = Vector4 {x: kx, y: ky, z: kz, w: kw};
    let w:Vector4 = Vector4 {x: wx, y: wy, z: wz, w: ww};

    return Mat4 {i: i, j: j, k: k, w: w};
}
