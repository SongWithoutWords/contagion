use crate::core::vector::*;

pub fn check_bounding_box(corner1: Vector2, corner2: Vector2, point_of_interest: Vector2) -> bool {
    let x_pos = point_of_interest.x;
    let y_pos = point_of_interest.y;
    let rec_min_x = corner1.x.min(corner2.x);
    let rec_min_y = corner1.y.min(corner2.y);
    let rec_max_x = corner1.x.max(corner2.x);
    let rec_max_y = corner1.y.max(corner2.y);
    return rec_min_x <= x_pos && rec_max_x >= x_pos && rec_min_y <= y_pos && rec_max_y >= y_pos ;
}