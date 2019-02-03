use crate::core::vector::*;
use crate::core::scalar::Scalar;

pub struct State {
    pub entities: Vec<Entity>,
    pub is_selected: Vec<bool>,
    pub projectiles: Vec<Projectile>,
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
pub const COP_RELOAD_COOLDOWN: Scalar = 10.0;
pub const COP_AIM_COOLDOWN: Scalar = 2.0;
pub const COP_MAGAZINE_CAPACITY: i64 = 6;

#[derive(Copy, Clone)]
pub enum Behaviour {
    Cop {
        rounds_in_magazine: i64,
        state: CopState,
    },
    Dead,
    Human,
    Zombie
}

pub const COP_MIN_DISTANCE_FROM_WAYPOINT_SQUARED: Scalar = 0.01;

#[derive(Copy, Clone)]
pub enum CopState {
    Aiming {
        aim_time_remaining: Scalar,
        target_index: usize,
    },
    Moving {
        waypoint: Vector2,
    },
    Reloading {
        reload_time_remaining: Scalar,
    },
    Idle
}

pub const BULLET_SPEED: f64 = 50.0;
pub const MIN_PROJECTILE_SPEED: f64 = 0.1;

pub struct Projectile {
    pub position: Vector2,
    pub velocity: Vector2,
}
