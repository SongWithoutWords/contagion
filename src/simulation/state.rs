use crate::core::vector::*;
use crate::core::scalar::Scalar;
use crate::core::geo::polygon::*;

use crate::simulation::ai::path::Path;

use std::collections::HashSet;

pub struct State {
    pub entities: Vec<Entity>,
    pub buildings: Vec<Polygon>,
    pub building_outlines: Vec<Polygon>,
    pub selection: HashSet<usize>,
    pub projectiles: Vec<Projectile>,
    pub rng: rand_xorshift::XorShiftRng,
}

pub const ENTITY_RADIUS: Scalar = 0.5;
pub const ENTITY_DRAG: Scalar = 1.0;

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
    pub fn accelerate_along_vector(&mut self, vector: Vector2, delta_time: Scalar, force: Scalar) {
        self.look_along_vector(vector, delta_time);
        self.velocity += delta_time * force * vector.normalize();
    }
}

pub const COP_MOVEMENT_FORCE: Scalar = 1.0;
pub const CIVILIAN_MOVEMENT_FORCE: Scalar = 1.0;
pub const ZOMBIE_MOVEMENT_FORCE: Scalar = 1.5;

pub const COP_RELOAD_COOLDOWN: Scalar = 4.0;
pub const COP_AIM_TIME_MEAN: Scalar = 1.0;

// Used only for log normal distribution, and we're presently using exponential distribution
// pub const COP_AIM_TIME_STD_DEV: Scalar = 1.0;

pub const COP_ANGULAR_ACCURACY_STD_DEV: Scalar = 0.1;

pub const COP_MAGAZINE_CAPACITY: i64 = 6;

#[derive(Clone, PartialEq)]
pub enum Behaviour {
    Cop {
        rounds_in_magazine: i64,
        state: CopState
    },
    Dead,
    Human,
    Zombie {
        state: ZombieState
    }
}

pub const COP_MIN_DISTANCE_FROM_WAYPOINT_SQUARED: Scalar = 0.01;

#[derive(Clone, PartialEq)]
pub enum CopState {
    Aiming {
        aim_time_remaining: Scalar,
        target_index: usize,
    },
    Moving {
        waypoint: Vector2,
        mode: MoveMode,
        path: Option<Path>
    },
    Reloading {
        reload_time_remaining: Scalar,
    },
    Idle
}

#[derive(Clone, PartialEq)]
pub enum ZombieState {
    Chasing {
        target_index: usize
    },
    Moving {
        waypoint: Vector2
    },
    Roaming
}

pub const PROJECTILE_DRAG: Scalar = 1.0;

#[derive(Copy, Clone, PartialEq)]
pub struct Projectile {
    pub position: Vector2,
    pub velocity: Vector2,
    pub kind: ProjectileKind,
}

pub const BULLET_RADIUS: Scalar = 0.12;
pub const BULLET_SPEED: Scalar = 40.0;
pub const BULLET_SPEED_MIN: Scalar = 10.0;
pub const BULLET_SPAWN_DISTANCE_MULTIPLIER: Scalar = 1.25;
pub const CASING_SPEED: Scalar = 1.0;

#[derive(Copy, Clone, PartialEq)]
pub enum ProjectileKind {
    Bullet,
    Casing,
}

#[derive(Copy, Clone, PartialEq)]
pub enum MoveMode {
    MoveAttacking,
    Sprinting
}
