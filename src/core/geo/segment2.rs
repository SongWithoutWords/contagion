use crate::core::vector::*;

pub struct Segment2 {
    pub p1: Vector2,
    pub p2: Vector2,
}

impl Segment2 {
    // Find the magnitude of the shortest distance from the given point to the segment
    pub fn dist_squared(&self, point: Vector2) -> f64 {
        ((self.p2.y - self.p1.y) * point.x - (self.p2.x - self.p1.x) * point.y + self.p2.x * self.p1.y - self.p2.y * self.p1.x).powf(2.0) /
            ((self.p2.y - self.p1.y).powf(2.0) + (self.p2.x - self.p1.x).powf(2.0))
    }
}
