use crate::core::scalar::*;
use crate::core::geo::circle::*;
use crate::core::geo::segment2::*;

pub struct IntersectionTimes(Scalar, Scalar);

// Returns the "times" of the intersection with the circle along the line segment
// Adapted from:
// https://stackoverflow.com/questions/1073336/circle-line-segment-collision-detection-algorithm
pub fn segment_circle_intersection_times(s: &Segment2, c: &Circle) -> Option<IntersectionTimes> {

    let a_to_b = s.p2 - s.p1;
    let c_to_a = s.p1 - c.center;

    let a = a_to_b.dot(a_to_b);
    let b = 2.0 * c_to_a.dot(a_to_b);
    let c = c_to_a.dot(c_to_a) - c.radius * c.radius;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        None
    } else {
        let disc_sqrt = discriminant.sqrt();
        Some(IntersectionTimes(
            0.5 * (-b - disc_sqrt) / a,
            0.5 * (-b + disc_sqrt) / a,
        ))
    }
}

pub fn segment_circle_has_intersection(s: &Segment2, c: &Circle) -> bool {
    match segment_circle_intersection_times(s, c) {
        None => false,
        Some(IntersectionTimes(t1, t2)) =>
            0.0 <= t1 && t1 <= 1.0 ||
            0.0 <= t2 && t2 <= 1.0
    }
}

pub fn segment_circle_first_intersection_time(s: &Segment2, c: &Circle) -> Option<Scalar> {
    match segment_circle_intersection_times(s, c) {
        None => None,
        Some(IntersectionTimes(t1, t2)) =>
            if 0.0 <= t1 && t1 <= 1.0 {
                Some(t1)
            } else
            if 0.0 <= t2 && t2 <= 1.0 {
                Some(t2)
            } else {
                None
            }
    }
}

pub fn segment_circle_first_intersection_point(s: &Segment2, c: &Circle) -> Option<Vector2> {
    match segment_circle_first_intersection_time(s, c) {
        None => None,
        Some(t) => Some(t * (s.p2 - s.p1))
    }
}

