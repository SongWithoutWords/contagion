use crate::core::scalar::Scalar;
use crate::core::vector::*;

#[derive(Clone, Copy, Debug)]
pub struct Mat4 {
    pub i: Vector4,
    pub j: Vector4,
    pub k: Vector4,
    pub w: Vector4
}

impl Mat4 {
    pub fn multiply_vec2(&self , vec: Vector2) -> Vector2 {
        let vec4: Vector4 = Vector4 { x: vec.x, y: vec.y, z: 0.0, w: 1.0 };

        let row1: Vector4 = Vector4 { x: self.i.x, y: self.j.x, z: self.k.x, w: self.w.x };
        let row2: Vector4 = Vector4 { x: self.i.y, y: self.j.y, z: self.k.y, w: self.w.y };

        let new_x = row1.dot(vec4);
        let new_y = row2.dot(vec4);

        return Vector2 { x: new_x, y: new_y };
    }

    pub fn inverse_matrix4(&self) -> Mat4 {
        let a2323 = self.k.z * self.w.w - self.k.w * self.w.z;
        let a1323 = self.k.y * self.w.w - self.k.w * self.w.y;
        let a1223 = self.k.y * self.w.z - self.k.z * self.w.y;
        let a0323 = self.k.x * self.w.w - self.k.w * self.w.x;
        let a0223 = self.k.x * self.w.z - self.k.z * self.w.x;
        let a0123 = self.k.x * self.w.y - self.k.y * self.w.x;
        let a2313 = self.j.z * self.w.w - self.j.w * self.w.z;
        let a1313 = self.j.y * self.w.w - self.j.w * self.w.y;
        let a1213 = self.j.y * self.w.z - self.j.z * self.w.y;
        let a2312 = self.j.z * self.k.w - self.j.w * self.k.z;
        let a1312 = self.j.y * self.k.w - self.j.w * self.k.y;
        let a1212 = self.j.y * self.k.z - self.j.z * self.k.y;
        let a0313 = self.j.x * self.w.w - self.j.w * self.w.x;
        let a0213 = self.j.x * self.w.z - self.j.z * self.w.x;
        let a0312 = self.j.x * self.k.w - self.j.w * self.k.x;
        let a0212 = self.j.x * self.k.z - self.j.z * self.k.x;
        let a0113 = self.j.x * self.w.y - self.j.y * self.w.x;
        let a0112 = self.j.x * self.k.y - self.j.y * self.k.x;

        let det: f64 = (self.i.x * (self.j.y * a2323 - self.j.z * a1323 + self.j.w * a1223)
            - self.i.y * (self.j.x * a2323 - self.j.z * a0323 + self.j.w * a0223)
            + self.i.z * (self.j.x * a1323 - self.j.y * a0323 + self.j.w * a0123)
            - self.i.w * (self.j.x * a1223 - self.j.y * a0223 + self.j.z * a0123));

        let ix = 1.0 / det * (self.j.y * a2323 - self.j.z * a1323 + self.j.w * a1223);
        let iy = 1.0 / det * -(self.i.y * a2323 - self.i.z * a1323 + self.i.w * a1223);
        let iz = 1.0 / det * (self.i.y * a2313 - self.i.z * a1313 + self.i.w * a1213);
        let iw = 1.0 / det * -(self.i.y * a2312 - self.i.z * a1312 + self.i.w * a1212);
        let jx = 1.0 / det * -(self.j.x * a2323 - self.j.z * a0323 + self.j.w * a0223);
        let jy = 1.0 / det * (self.i.x * a2323 - self.i.z * a0323 + self.i.w * a0223);
        let jz = 1.0 / det * -(self.i.x * a2313 - self.i.z * a0313 + self.i.w * a0213);
        let jw = 1.0 / det * (self.i.x * a2312 - self.i.z * a0312 + self.i.w * a0212);
        let kx = 1.0 / det * (self.j.x * a1323 - self.j.y * a0323 + self.j.w * a0123);
        let ky = 1.0 / det * -(self.i.x * a1323 - self.i.y * a0323 + self.i.w * a0123);
        let kz = 1.0 / det * (self.i.x * a1313 - self.i.y * a0313 + self.i.w * a0113);
        let kw = 1.0 / det * -(self.i.x * a1312 - self.i.y * a0312 + self.i.w * a0112);
        let wx = 1.0 / det * -(self.j.x * a1223 - self.j.y * a0223 + self.j.z * a0123);
        let wy = 1.0 / det * (self.i.x * a1223 - self.i.y * a0223 + self.i.z * a0123);
        let wz = 1.0 / det * -(self.i.x * a1213 - self.i.y * a0213 + self.i.z * a0113);
        let ww = 1.0 / det * (self.i.x * a1212 - self.i.y * a0212 + self.i.z * a0112);

        let i: Vector4 = Vector4 { x: ix, y: iy, z: iz, w: iw };
        let j: Vector4 = Vector4 { x: jx, y: jy, z: jz, w: jw };
        let k: Vector4 = Vector4 { x: kx, y: ky, z: kz, w: kw };
        let w: Vector4 = Vector4 { x: wx, y: wy, z: wz, w: ww };

        return Mat4 { i: i, j: j, k: k, w: w };
    }

    pub fn as_f32_array(&self) -> [[f32; 4]; 4] {
        [
            [self.i.x as f32, self.i.y as f32, self.i.z as f32, self.i.w as f32],
            [self.j.x as f32, self.j.y as f32, self.j.z as f32, self.j.w as f32],
            [self.k.x as f32, self.k.y as f32, self.k.z as f32, self.k.w as f32],
            [self.w.x as f32, self.w.y as f32, self.w.z as f32, self.w.w as f32],
        ]
    }
}
