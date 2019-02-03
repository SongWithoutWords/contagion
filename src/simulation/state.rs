use crate::core::vector::*;
use crate::core::scalar::Scalar;

pub struct State {
    pub entities: Vec<Entity>,
    pub is_selected: Vec<bool>
}

pub const ENTITY_RADIUS: Scalar = 0.5;

pub struct Entity {
    pub position: Vector2,
    pub velocity: Vector2,
    pub behaviour: Behaviour
}

impl Entity {
    pub fn get_facing_normal(&self) -> Vector2 {
        return self.velocity.normalize();
    }
}

pub enum Behaviour {
    Cop { waypoint: Option<Vector2> },
    Dead,
    Human,
    Zombie
}
