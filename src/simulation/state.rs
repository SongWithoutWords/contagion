use crate::core::vector::*;
use crate::core::scalar::Scalar;
use crate::core::geo::polygon::*;

pub struct State {
    pub entities: Vec<Entity>,
    pub buildings: Vec<Polygon>,
    pub is_selected: Vec<bool>,
    pub projectiles: Vec<Projectile>,
    pub rng: rand_xorshift::XorShiftRng,
}

pub const ENTITY_RADIUS: Scalar = 0.5;

pub struct Entity {
    pub position: Vector2,
    pub velocity: Vector2,
    pub facing_angle: Scalar,
    pub behaviour: Behaviour,
}

impl Entity {
    pub fn get_facing_normal(&self) -> Vector2 {
        Vector2::from_angle(self.facing_angle)
    }
    pub fn look_along_angle(&mut self, angle: Scalar, delta_time: Scalar) {
        let delta_theta = (angle - self.facing_angle) % 6.28;
        let angular_deviation = if delta_theta < 3.14 { delta_theta } else { delta_theta - 6.28 };
        self.facing_angle += delta_time * angular_deviation;
    }
    pub fn look_along_vector(&mut self, vector: Vector2, delta_time: Scalar) {
        self.look_along_angle(vector.angle(), delta_time);
    }
    pub fn look_at_point(&mut self, point: Vector2, delta_time: Scalar) {
        self.look_along_vector(point - self.position, delta_time);
    }
    pub fn accelerate_along_vector(&mut self, vector: Vector2, delta_time: Scalar) {
        self.look_along_vector(vector, delta_time);
        self.velocity += vector.normalize_to(delta_time);
    }
}
pub const COP_RELOAD_COOLDOWN: Scalar = 10.0;
pub const COP_AIM_TIME_MEAN: Scalar = 1.0;
pub const COP_AIM_TIME_STD_DEV: Scalar = 1.0;
pub const COP_ANGULAR_ACCURACY_STD_DEV: Scalar = 0.1;

pub const COP_MAGAZINE_CAPACITY: i64 = 6;

#[derive(Copy, Clone, PartialEq)]
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

#[derive(Copy, Clone, PartialEq)]
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
