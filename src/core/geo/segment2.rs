use crate::core::vector::*;
use num::*;

pub struct Segment2 {
    pub p1: Vector2,
    pub p2: Vector2,
}

impl Segment2 {
    // Find the point on this segment that is nearest to the given point
    pub fn nearest_point_on_segment_to_point(&self, point: Vector2) -> Vector2 {
        let p1_to_p2 = self.p2 - self.p1;
        let p1_to_p = point - self.p1;

        let length_squared = p1_to_p2.length_squared();

        let dot_product = p1_to_p2.dot(p1_to_p);

        let time = clamp(dot_product / length_squared, 0.0, 1.0);

        self.p1 + p1_to_p2 * time
    }

    // Find minimum the distance from this segment to the given point
    pub fn distance_from_segment_to_point_squared(&self, point: Vector2) -> f64 {
        let nearest_point = self.nearest_point_on_segment_to_point(point);
        (nearest_point - point).length_squared()
    }

    // Find the point on the ray corresponding to this segment that is nearest to the given point
    pub fn nearest_point_on_ray_to_point(&self, point: Vector2) -> Vector2 {
        let p1_to_p2 = self.p2 - self.p1;
        let p1_to_p = point - self.p1;

        let length_squared = p1_to_p2.length_squared();

        let dot_product = p1_to_p2.dot(p1_to_p);

        let time = dot_product / length_squared;

        self.p1 + p1_to_p2 * time
    }

    // Find the minimum distance from the ray corresponding to this segment to the given point
    pub fn distance_from_ray_to_point_squared(&self, point: Vector2) -> f64 {
        let nearest_point = self.nearest_point_on_segment_to_point(point);
        (nearest_point - point).length_squared()
    }
}
