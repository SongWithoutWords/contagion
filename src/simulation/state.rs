use crate::core::vector::*;
use crate::core::scalar::Scalar;
use crate::core::geo::polygon::*;

use crate::simulation::ai::path::Path;
use crate::simulation::barricade::*;

use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Clone)]
pub struct State {
    pub entities: Vec<Entity>,
    pub buildings: Vec<Polygon>,
    pub building_outlines: Vec<Polygon>,
    pub building_type: HashMap<usize, u32>,
    pub barricades: Vec<Barricade>,
    pub selection: HashSet<usize>,
    pub projectiles: Vec<Projectile>,
    pub rng: rand_xorshift::XorShiftRng,
    pub money: u32
}

pub const ENTITY_RADIUS: Scalar = 0.5;
pub const ENTITY_DRAG: Scalar = 1.0;

#[derive(Clone)]
pub struct Entity {
    pub position: Vector2,
    pub velocity: Vector2,
    pub facing_angle: Scalar,
    pub dead_or_alive: DeadOrAlive,
}

impl Entity {
    // Transformation/orientation functions
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

    // Query functions
    pub fn is_dead(&self) -> bool {
        match self.dead_or_alive {
            DeadOrAlive::Dead => true,
            _ => false
        }
    }
    pub fn is_alive(&self) -> bool {
        match self.dead_or_alive {
            DeadOrAlive::Alive { .. } => true,
            _ => false
        }
    }
    pub fn is_zombie(&self) -> bool {
        match self.dead_or_alive {
            DeadOrAlive::Alive { zombie_or_human: ZombieOrHuman::Zombie { .. }, .. } => true,
            _ => false
        }
    }
    pub fn is_human(&self) -> bool {
        match self.dead_or_alive {
            DeadOrAlive::Alive { zombie_or_human: ZombieOrHuman::Human { .. }, .. } => true,
            _ => false
        }
    }
    pub fn is_cop(&self) -> bool {
        match self.dead_or_alive {
            DeadOrAlive::Alive {
                zombie_or_human: ZombieOrHuman::Human {
                    human: Human::Cop { .. },
                    .. },
                ..
            } => true,
            _ => false
        }
    }
}

pub const COP_MOVEMENT_FORCE: Scalar = 1.5;
pub const CIVILIAN_MOVEMENT_FORCE: Scalar = 1.0;
pub const ZOMBIE_MOVEMENT_FORCE: Scalar = 1.6;

// pub const COP_RELOAD_COOLDOWN: Scalar = 4.0;
// pub const COP_AIM_TIME_MEAN: Scalar = 1.0;

// Used only for log normal distribution, and we're presently using exponential distribution
// pub const COP_AIM_TIME_STD_DEV: Scalar = 1.0;

pub const COP_MIN_DISTANCE_FROM_WAYPOINT_SQUARED: Scalar = 0.2;

// pub const COP_ANGULAR_ACCURACY_STD_DEV: Scalar = 0.01;

// pub const COP_MAGAZINE_CAPACITY: i64 = 6;

pub const ZOMBIE_SIGHT_RADIUS: f64 = 30.0;
pub const ZOMBIE_SIGHT_RADIUS_SQUARE: f64 = ZOMBIE_SIGHT_RADIUS * ZOMBIE_SIGHT_RADIUS;

pub const ZOMBIE_HUMAN_COLLISION_INFECTION_RATE: f64 = 0.01;

pub const HUMAN_SIGHT_RADIUS: f64 = 40.0;
pub const HUMAN_SIGHT_RADIUS_SQUARE: f64 = HUMAN_SIGHT_RADIUS * HUMAN_SIGHT_RADIUS;

pub const COP_SIGHT_RADIUS: f64 = 50.0;
pub const COP_SIGHT_RADIUS_SQUARE: f64 = COP_SIGHT_RADIUS * COP_SIGHT_RADIUS;

pub const ENTITY_HEALTH_MIN: f64 = 0.0;
pub const ENTITY_HEALTH_MAX: f64 = 1.0;

pub const INFECTION_MIN: f64 = 0.0;
pub const INFECTION_EXPONENTIAL_GROWTH_THRESHOLD: f64 = 0.1;
pub const INFECTION_EXPONENTIAL_GROWTH_RATE: f64 = 0.1;
pub const INFECTION_MAX: f64 = 1.0;

pub const FIGHTING_RANGE: Scalar = 1.0;
pub const ANGULAR_ACCURACY_STD_DEV: Scalar = 0.1;

pub const ZOMBIE_PUNCH_TIME: Scalar = 1.0;
pub const PUNCH_TIME: Scalar = 2.0;
pub const PUNCH_TIME_COOLDOWN: Scalar = 5.0;


#[derive(Clone)]
pub enum DeadOrAlive {
    Dead,
    Alive {
        health: Scalar,
        zombie_or_human: ZombieOrHuman
    }
}

#[derive(Clone)]
pub enum ZombieOrHuman {
    Zombie {
        state: ZombieState,
        left_hand_status: HandStatus,
        right_hand_status: HandStatus
    },
    Human {
        infection: Scalar,
        human: Human
    }
}

#[derive(Clone)]
pub enum Human {
    Civilian {
        state: HumanState,
        punch_time_cooldown: Scalar,
        left_hand_status: HandStatus,
        right_hand_status: HandStatus
    },
    Cop {
        cop_type: CopType,
        rounds_in_magazine: i64,
        state_stack: Vec<CopState>,
    },
}

#[derive(Copy, Clone)]
pub enum CopType {
    Normal,
    Soldier,
}
impl CopType {
    pub fn reload_time(self) -> Scalar {
        match self {
            CopType::Normal => 4.0,
            CopType::Soldier => 4.0,
        }
    }
    pub fn aim_time_mean(self) -> Scalar {
        match self {
            CopType::Normal => 2.0,
            CopType::Soldier => 5.0,
        }
    }
    pub fn magazine_capacity(self) -> i64 {
        match self {
            CopType::Normal => 6,
            CopType::Soldier => 20,
        }
    }
    pub fn angular_accuracy_std_dev(self) -> Scalar {
        match self {
            CopType::Normal => 0.01,
            CopType::Soldier => 0.005
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
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
    AttackingZombie {
        target_index: usize,
        path: Option<Path>,
    },
}

#[derive(Clone, PartialEq)]
pub enum ZombieState {
    Chasing {
        target_index: usize
    },
    Moving {
        waypoint: Vector2
    },
    Roaming {
        jerk: Vector2,
        acceleration: Vector2
    },
    Fighting {
        punch_time_remaining: Scalar,
        target_index: usize
    }
}

#[derive(Clone, PartialEq)]
pub enum HumanState {
    Running,
    Fighting {
        target_index: usize,
        punch_time_remaining: Scalar
    }
}

pub const PROJECTILE_DRAG: Scalar = 0.2;

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

pub const BULLET_DAMAGE_MIN: Scalar = 0.25;
pub const BULLET_DAMAGE_MAX: Scalar = 1.0;

pub const BULLET_MAX_DAMAGE_DISTANCE_FROM_ENTITY_CENTER: Scalar = 0.25 * ENTITY_RADIUS;
pub const BULLET_MIN_DAMAGE_DISTANCE_FROM_ENTITY_CENTER: Scalar = 1.0 * ENTITY_RADIUS;

pub const FIST_RADIUS: Scalar = 0.3;
pub const FIST_SPEED: Scalar = 0.6;
pub const FIST_SPEED_MIN: Scalar = 0.3;
pub const FIST_SPAWN_DISTANCE_MULTIPLIER: Scalar = 1.25;
pub const FIST_DAMAGE: Scalar = 0.50;


// When left hand is true, hand generated is left, else hand generated is right
#[derive(Copy, Clone, PartialEq)]
pub enum ProjectileKind {
    Bullet,
    Casing,
    Fist {
        owner_index: usize,
        left_hand: bool,
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MoveMode {
    Moving,
    Sprinting
}

#[derive(Clone, PartialEq)]
pub enum HandStatus {
    None,
    Normal
}
