use crate::core::vector::*;
use crate::core::scalar::*;
use crate::core::geo::polygon::*;
use crate::simulation::state::ENTITY_RADIUS;

#[derive(Clone, Debug)]
pub struct Barricade {
    pub poly: Polygon,
    pub outline: Polygon,
    pub health: Scalar
}

impl Barricade {
    pub fn new(start: Vector2, end: Vector2) -> Barricade {
        let poly = barricade_poly(start, end);

        let mut outlines = vec!();
        let norms = poly.normals();

        for i in 0..poly.num_sides() {
            let norm_sum = norms[i] + norms[if i < 1 { poly.num_sides() - 1 } else { i - 1 }];
            let offset = norm_sum * ENTITY_RADIUS * 1.1;
            outlines.push(offset + poly.get(i));
        }

        Barricade {
            poly,
            outline: Polygon(outlines),
            health: 100.0
        }
    }
}

pub fn barricade_poly(start: Vector2, end: Vector2) -> Polygon {
    let normal = (end - start).right() / ((end - start).length() * 4.0);
    Polygon(vec![start + normal, start - normal, end - normal, end + normal])
}
