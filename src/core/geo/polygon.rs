use crate::core::vector::*;
use crate::core::scalar::*;

// Could probably refactor this to be a Vec<Edge>
// Polygon is an ordered vec of vertices (represented by Vector2)
#[derive(Clone, Debug)]
pub struct Polygon(pub Vec<Vector2>);

impl Polygon {
    pub fn num_sides(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, i: usize) -> Vector2 {
        self.0[i]
    }

    // Find the normals of all edges of the polygon
    pub fn normals(&self) -> Vec<Vector2> {
        let mut out = Vec::new();

        for i in 0..self.num_sides() {
            let ab = self.0[(i + 1) % self.num_sides()] - self.0[i];
            let ac = self.0[if i < 1 { self.num_sides()-1 } else { i-1 }] - self.0[i];
            let n = Vector2 { x: ab.y, y: -ab.x }.normalize();
            let d = n.dot(ac);

            if d > 0.0 {
                out.push(-n);
            }
            else {
                out.push(n);
            }
        }

        out
    }

    pub fn triangles(&self) -> Vec<Polygon> {
        if self.num_sides() < 3 { panic!("Polygons must have at least 3 sides!") }
        if self.num_sides() == 3 { return vec![self.clone()] }

        let triangle = Polygon(vec![self.get(0), self.get(1), self.get(2)]);

        let mut vertices = self.0.clone();
        vertices.remove(1);

        let mut triangles = Polygon(vertices).triangles();
        triangles.push(triangle);
        triangles
    }

    pub fn bounding_box(&self) -> (Vector2, Vector2) {
        let mut max = Vector2 { x: NEG_INFINITY, y: NEG_INFINITY };
        let mut min = Vector2 { x: INFINITY, y: INFINITY };

        for point in self.0.clone() {
            if point.x < min.x { min.x = point.x }
            if point.y < min.y { min.y = point.y }
            if point.x > max.x { max.x = point.x }
            if point.y > max.y { max.y = point.y }
        }

        if max.x <= min.x || max.y <= min.y { panic!("Invalid polygon bounding box!") }

        (min, max)
    }

    // Find the number of intersections with the line spanned by start and end
    pub fn num_intersects(&self, start: Vector2, end: Vector2) -> usize {
        self.intersects(start, end).len()
    }

    // Find the position of all intersections of the line spanned by start and end
    pub fn intersects(&self, start: Vector2, end: Vector2) -> Vec<Vector2> {
        let mut out = Vec::new();

        // Vertical line
        if start.x == end.x {
            // For each side...
            for i in 0..self.num_sides() {
                let p1 = self.get(i);
                let p2 = self.get((i + 1) % self.num_sides());

                // Lines aren't both vertical and line is within x range of the side i
                if p1.x != p2.x && p1.x.min(p2.x) <= start.x && p1.x.max(p2.x) >= start.x {
                    // Find the y that the lines intersect at
                    let m = (p2.y - p1.y) / (p2.x - p1.x);
                    let b = p1.y - m * p1.x;
                    let intersect_y = m * start.x + b;

                    // If it is in the y range of the line, push to output
                    if start.y.min(end.y) <= intersect_y && start.y.max(end.y) >= intersect_y {
                        out.push(Vector2 { x: start.x, y: intersect_y });
                    }
                }
            }
        } else {
            // Find equation of input line
            let m1 = (end.y - start.y) / (end.x - start.x);
            let b1 = start.y - m1 * start.x;

            // For each side...
            for i in 0..self.num_sides() {
                let p1 = self.get(i);
                let p2 = self.get((i + 1) % self.num_sides());

                // Side is vertical
                if p1.x == p2.x {
                    // Ensure that side is in x range of input line
                    if start.x.min(end.x) <= p1.x && start.x.max(end.x) >= p1.x {
                        // Find the y that the lines intersect at
                        let intersect_y = m1 * p1.x + b1;

                        // If intersect is in the y range of the side, push to output
                        if p1.y.min(p2.y) <= intersect_y && p1.y.max(p2.y) >= intersect_y {
                            out.push(Vector2 { x: p1.x, y: intersect_y });
                        }
                    }
                } else {
                    // Find equation of the side
                    let m2 = (p2.y - p1.y) / (p2.x - p1.x);
                    let b2 = p1.y - m2 * p1.x;

                    // Ensure side and line are not parallel
                    if m1 != m2 {
                        // Find x and y intersection
                        let intersect_x = (b2 - b1) / (m1 - m2);
                        let intersect_y = m1 * intersect_x + b1;

                        // Make sure the intersection is on both the side and the line
                        if intersect_x >= start.x.min(end.x) && intersect_x <= start.x.max(end.x) &&
                            intersect_y >= start.y.min(end.y) && intersect_y <= start.y.max(end.y) &&
                            p1.x.min(p2.x) <= intersect_x && p1.x.max(p2.x) >= intersect_x &&
                            p1.y.min(p2.y) <= intersect_y && p1.y.max(p2.y) >= intersect_y {

                            out.push(Vector2 { x: intersect_x, y: intersect_y });
                        }
                    }
                }
            }
        }

        out
    }

    pub fn contains_point(&self, pos: Vector2) -> bool {
        self.num_intersects(pos, Vector2 { x: pos.x, y: MAX }) % 2 == 1
    }
}
