use crate::core::vector::*;

// Could probably refactor this to be a Vec<Edge>
#[derive(Clone, Debug)]
pub struct Polygon(Vec<Vector2>);

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
            let ac = self.0[(i - 1) % self.num_sides()] - self.0[i];
            let n = -ac;
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

    // Find the position of all intersections of the line spanned by start and end
    pub fn intersects(&self, start: Vector2, end: Vector2) -> Vec<Vector2> {
        let mut out = Vec::new();

        let m1 = (start.y - end.y) / (start.x - end.x);
        let b1 = start.y - m1 * start.x;

        for i in 0..self.num_sides()-1 {
            let r = self.0[i];
            let s = self.0[(i + 1) % self.num_sides()];
            let m2 = (r.y - s.y) / (r.x - s.x);
            let b2 = r.y - m2 * r.x;

            if m1*b2 - m2*b1 != 0.0 {
                let x_intercept = (b2 - b1) / (m1 - m2);
                let y_intercept = m1 * x_intercept + b1;

                if x_intercept >= start.x.min(end.x) && x_intercept <= start.x.max(end.x) &&
                    y_intercept >= start.y.min(end.y) && y_intercept <= start.y.min(end.y) {
                    out.push(Vector2 { x: x_intercept, y: y_intercept })
                }
            }
        }

        out
    }
}
