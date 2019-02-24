use crate::core::vector::*;
use num::*;

pub struct Segment2 {
    pub p1: Vector2,
    pub p2: Vector2,
}

impl Segment2 {
    // Find the magnitude of the shortest distance from the given point to the segment

    pub fn nearest_point_to_point(&self, point: Vector2) -> Vector2 {
        let p1_to_p2 = self.p2 - self.p1;
        let p1_to_p = point - self.p1;

        let length_squared = p1_to_p2.length_squared();

        let dot_product = p1_to_p2.dot(p1_to_p);

        let time = clamp(dot_product / length_squared, 0.0, 1.0);

        // let time = (dot_product / length_squared).clamp(0.0, 1.0);
        self.p1 + p1_to_p2 * time
    }

    pub fn dist_squared(&self, point: Vector2) -> f64 {
        let nearest_point = self.nearest_point_to_point(point);
        (nearest_point - point).length_squared()
    }
}
