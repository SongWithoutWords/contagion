use crate::core::vector::*;
use crate::core::scalar::*;
use crate::core::geo::polygon::*;
use crate::simulation::state::*;

pub const BARRICADE_HEALTH: Scalar = 100.0;
pub const BARRICADE_MIN_COST: u32 = 5;

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
            health: BARRICADE_HEALTH
        }
    }
}

pub fn barricade_poly(start: Vector2, end: Vector2) -> Polygon {
    let normal = (end - start).right() / ((end - start).length() * 4.0);
    Polygon(vec![start + normal, start - normal, end - normal, end + normal])
}

pub fn barricade_valid(start: Vector2, end: Vector2, state: &State) -> bool {
    let cost = barricade_cost(start, end);
    cost >= BARRICADE_MIN_COST && cost <= state.money &&
        barricade_state_valid(barricade_poly(start, end), state)
}

pub fn barricade_cost(start: Vector2, end: Vector2) -> u32 {
    (end - start).length().round() as u32
}

fn barricade_state_valid(poly: Polygon, state: &State) -> bool {
    for building in state.buildings.clone() {
        for i in 0..poly.num_sides() {
            if building.contains_point(poly.get(i)) ||
                building.num_intersects(poly.get(i), poly.get((i + 1) % poly.num_sides())) > 0 {
                return false
            }
        }
    }

    for barricade in state.barricades.clone() {
        for i in 0..poly.num_sides() {
            if barricade.poly.contains_point(poly.get(i)) ||
                barricade.poly.num_intersects(poly.get(i), poly.get((i + 1) % poly.num_sides())) > 0 {
                return false
            }
        }
    }

    true
}
