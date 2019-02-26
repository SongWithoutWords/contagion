extern crate music;

use rand::distributions::*;

use crate::core::geo::circle::*;
use crate::core::geo::intersect::segment_circle::*;
use crate::core::geo::polygon::*;
use crate::core::geo::segment2::*;

use crate::simulation::ai::pathfinding::find_path;
use crate::simulation::ai::path::Path;

use crate::presentation::audio::sound_effects::*;

use super::state::*;

pub struct UpdateArgs {
    pub dt: Scalar
}


pub fn update(args: &UpdateArgs, state: &mut State) {


    // Apply individual behaviours
    for i in 0..state.entities.len() {
        match &state.entities[i].behaviour {
            b @ Behaviour::Cop { .. } => {
                // simulate_cop(args, &mut entities, i),
                let behaviour = update_cop(&args, state, i, b.clone());
                state.entities[i].behaviour = behaviour
            }
            Behaviour::Dead =>
            // Do nothing
                (),
            Behaviour::Human =>
            // Run from zombies!
                simulate_human(args, &mut state.entities, i),
            Behaviour::Zombie =>
            // Chase humans and cops!
                simulate_zombie(args, state, i)
        }
    }

    const DOUBLE_ENTITY_RADIUS_SQUARED: f64 = 4.0 * ENTITY_RADIUS * ENTITY_RADIUS;

    // Check for collisions
    for i in 0..state.entities.len() {
        let p1 = state.entities[i].position;
        let circle = Circle { center: p1, radius: ENTITY_RADIUS };

        // Collisions with other entities
        for j in (i + 1)..state.entities.len() {
            let p2 = state.entities[j].position;

            let delta = p2 - p1;

            let delta_length_squared = delta.length_squared();

            if delta_length_squared < DOUBLE_ENTITY_RADIUS_SQUARED {
                handle_collision(args, &mut state.entities, i, j, &delta, delta_length_squared);
            }
        }

        // Collisions with buildings
        for j in 0..state.buildings.len() {
            // Check if position is inside the building
            let start = Vector2 { x: p1.x, y: p1.y };
            let end   = Vector2 { x: p1.x, y: MAX };
            let mut overlap = state.buildings[j].num_intersects(start, end) % 2 == 1;
            let inside = overlap;

            // Don't bother doing this if we already know there's an overlap
            if !inside {
                // Check if one of the building's sides intersects the entity
                for k in 0..state.buildings[j].num_sides() {
                    let segment = Segment2 {
                        p1: state.buildings[j].get(k),
                        p2: state.buildings[j].get((k + 1) % state.buildings[j].num_sides())
                    };

                    if segment_circle_has_intersection(&segment, &circle) {
                        overlap = true;
                        break;
                    }
                }
            }

            if overlap {
                handle_building_collision(args, &mut state.entities[i], &state.buildings[j], inside);
                break;
            }
        }
    }

    // Apply acceleration
    for e in &mut state.entities {
        let displacement = args.dt * e.velocity;
        e.position += displacement;
        e.velocity -= ENTITY_DRAG * displacement;
    }

    // Remove motionless bullets
    state.projectiles.retain(
        |p| p.kind != ProjectileKind::Bullet ||
            p.velocity.length_squared() > BULLET_SPEED_MIN
    );

    // Update projectiles
    for p in &mut state.projectiles {

        let displacement = args.dt * p.velocity;
        p.velocity -= PROJECTILE_DRAG * displacement;

        let segment = Segment2 { p1: p.position, p2: p.position + displacement };

        p.position = segment.p2;

        if p.kind != ProjectileKind::Bullet {
            continue;
        }

        let mut first_intersect_time_and_index = None;
        for i in 0..state.entities.len() {
            let entity = &state.entities[i];

            if entity.behaviour == Behaviour::Dead {
                // Dead entities don't collide with bullets
                continue;
            }

            let circle = Circle { center: entity.position, radius: ENTITY_RADIUS };

            let this_min_intersection = segment_circle_min_positive_intersect_time(&segment, &circle);

            match (first_intersect_time_and_index, this_min_intersection) {
                (None, Some(this)) => {
                    first_intersect_time_and_index = Some((this, i))
                }
                (Some((min, _)), Some(this)) if this < min => {
                    first_intersect_time_and_index = Some((this, i))
                }
                _ => ()
            }
        }

        match first_intersect_time_and_index {
            None => (),
            Some((_, i)) => {
                state.entities[i].behaviour = Behaviour::Dead;
                p.velocity = Vector2::zero();
                play_zombie_dead();
            }
        }
    }
}

fn handle_collision(
    args: &UpdateArgs,
    entities: &mut Vec<Entity>,
    i: usize,
    j: usize,
    delta: &Vector2,
    delta_length_squared: f64) {

    // Spread the infection from zombies to others
    match (&entities[i].behaviour, &entities[j].behaviour) {
        (Behaviour::Human, Behaviour::Zombie) | (Behaviour::Cop { .. }, Behaviour::Zombie) => {
            entities[i].behaviour = Behaviour::Zombie;
            play_person_infected();
        }
        (Behaviour::Zombie, Behaviour::Human) | (Behaviour::Zombie, Behaviour::Cop { .. }) => {
            entities[j].behaviour = Behaviour::Zombie;
            play_person_infected()
        }
        _ => ()
    }

    // Force entities apart that are overlapping
    let velocity_change = *delta * (args.dt / delta_length_squared);
    entities[i].velocity -= velocity_change;
    entities[j].velocity += velocity_change;
}

fn handle_building_collision(
    args: &UpdateArgs,
    entity: &mut Entity,
    building: &Polygon,
    inside: bool) {

    let mut distance_squared = INFINITY;
    let mut normal = Vector2::zero();
    let normals = building.normals();

    // Find the closest edge
    for i in 0..building.num_sides() {
        let seg_i = Segment2 {
            p1: building.get(i),
            p2: building.get((i + 1) % building.num_sides())
        };
        let dist_i = seg_i.dist_squared(entity.position);

        if distance_squared > dist_i {
            distance_squared = dist_i;
            normal = normals[i];
        }
    }

    const SPRING_CONSTANT: f64 = 32.0;

    let distance = distance_squared.sqrt();

    if inside {
        // If the entity is inside move them to the nearest edge
        entity.position += (distance + ENTITY_RADIUS) * normal;
    } else {
        // If the entity is overlapping, force them away from the edge
        let overlap = ENTITY_RADIUS - distance;
        entity.velocity += args.dt * SPRING_CONSTANT * overlap * normal
    }
}

fn update_cop(
    args: &UpdateArgs,
    sim_state: &mut State,
    index: usize,
    behaviour: Behaviour) -> Behaviour {

    let entities = &mut sim_state.entities;
    let buildings = &sim_state.buildings;
    let building_outlines = &sim_state.building_outlines;

    match behaviour {
        Behaviour::Cop { rounds_in_magazine, state } => {
            match state {
                CopState::Aiming { mut aim_time_remaining, target_index } => {
                    if entities[target_index].behaviour == Behaviour::Dead {
                        // Stop aiming if the target is already dead
                        return Behaviour::Cop {
                            rounds_in_magazine: rounds_in_magazine,
                            state: CopState::Idle,
                        };
                    }
                    // TODO: check if we can still see the target, and stop aiming if not

                    let my_pos = entities[index].position;
                    let target_pos = entities[target_index].position;
                    let delta = target_pos - my_pos;
                    entities[index].look_along_vector(delta, args.dt);

                    aim_time_remaining -= args.dt;
                    if aim_time_remaining > 0.0 {
                        // Taking aim, do nothing
                        Behaviour::Cop {
                            rounds_in_magazine: rounds_in_magazine,
                            state: CopState::Aiming { aim_time_remaining, target_index: target_index },
                        }
                    } else {
                        let angular_deviation =
                            Normal::new(0.0, COP_ANGULAR_ACCURACY_STD_DEV).sample(&mut sim_state.rng);

                        // Finished aiming, take the shot
                        let delta_normal = delta.rotate_by(angular_deviation);

                        // Spawn outside of the entity - don't want to shoot the entity itself
                        let spawn_pos = entities[index].position +
                            BULLET_SPAWN_DISTANCE_MULTIPLIER * ENTITY_RADIUS * delta_normal;

                        // Fire at the taget
                        sim_state.projectiles.push(
                            Projectile {
                                position: spawn_pos,
                                velocity: BULLET_SPEED * delta_normal,
                                kind: ProjectileKind::Bullet
                            });

                        sim_state.projectiles.push(
                            Projectile {
                                position: spawn_pos,
                                // Casing ejects from the right of the weapon
                                velocity: CASING_SPEED * delta_normal.right(),
                                kind: ProjectileKind::Casing
                            });

                        play_shotgun();

                        Behaviour::Cop {
                            rounds_in_magazine: rounds_in_magazine - 1,
                            state: CopState::Idle,
                        }
                    }
                }
                CopState::Moving { waypoint } => {
                    match find_path(entities[index].position, waypoint, buildings, building_outlines) {
                        None => {
                            Behaviour::Cop {
                                rounds_in_magazine: rounds_in_magazine,
                                state: CopState::Idle
                            }
                        },
                        Some(path) => {
                            match path.to_vec().get(1) {
                                None => Behaviour::Cop {
                                    rounds_in_magazine: rounds_in_magazine,
                                    state: CopState::Idle
                                },
                                Some(&node) => {
                                    let delta = node - entities[index].position;

                                    if waypoint == node && delta.length_squared() < COP_MIN_DISTANCE_FROM_WAYPOINT_SQUARED {
                                        Behaviour::Cop {
                                            rounds_in_magazine: rounds_in_magazine,
                                            state: CopState::Idle
                                        }
                                    } else {
                                        entities[index].accelerate_along_vector(delta, args.dt);
                                        Behaviour::Cop {
                                            rounds_in_magazine: rounds_in_magazine,
                                            state: CopState::Moving { waypoint }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                CopState::Reloading { reload_time_remaining } => {
                    let half_reload_time = 0.5 * COP_RELOAD_COOLDOWN;
                    let new_reload_time_remaining = reload_time_remaining - args.dt;

                    // Play the reload sound when half-done reloading
                    if reload_time_remaining > half_reload_time &&
                        half_reload_time > new_reload_time_remaining {
                        play_reload();
                    }

                    if reload_time_remaining > 0.0 {
                        // Reloading, do nothing
                        Behaviour::Cop {
                            rounds_in_magazine: rounds_in_magazine,
                            state: CopState::Reloading {
                                reload_time_remaining: new_reload_time_remaining
                            },
                        }
                    } else {
                        // Finished reloading, replenish rounds
                        Behaviour::Cop {
                            rounds_in_magazine: COP_MAGAZINE_CAPACITY,
                            state: CopState::Idle,
                        }
                    }
                }
                CopState::Idle => {
                    // Reload if you don't have ammo
                    if rounds_in_magazine <= 0 {
                        Behaviour::Cop {
                            rounds_in_magazine: rounds_in_magazine,
                            state: CopState::Reloading { reload_time_remaining: COP_RELOAD_COOLDOWN },
                        }
                    }
                    // Look for target if you do have ammo
                    else {
                        let my_pos = sim_state.entities[index].position;

                        let mut min_index = 0;
                        let mut min_distance_sqr = INFINITY;

                        for i in 0..sim_state.entities.len() {
                            match sim_state.entities[i].behaviour {

                                // Target zombies
                                Behaviour::Zombie => {
                                    let delta = sim_state.entities[i].position - my_pos;
                                    let distance_sqr = delta.length_squared();
                                    if distance_sqr < min_distance_sqr {
                                        min_index = i;
                                        min_distance_sqr = distance_sqr;
                                    }
                                }

                                // Skip everything else
                                _ => ()
                            }
                        }

                        if min_distance_sqr < INFINITY {
                            let aim_time_distribution =
                                Exp::new(COP_AIM_TIME_MEAN);
                            Behaviour::Cop {
                                rounds_in_magazine: rounds_in_magazine,
                                state: CopState::Aiming {
                                    aim_time_remaining: aim_time_distribution.sample(&mut sim_state.rng),
                                    target_index: min_index,
                                },
                            }
                        } else {
                            // Remain in idle state
                            behaviour
                        }
                    }
                }
            }
        }
        _ => panic!("Entity at index should be a cop!")
    }
}

fn simulate_zombie(args: &UpdateArgs, sim_state: &mut State, index: usize) {

    let entities = &mut sim_state.entities;
    let buildings = &sim_state.buildings;
    let building_outlines = &sim_state.building_outlines;

    let my_pos = entities[index].position;

    let mut min_path: Option<Path> = None;
    let mut min_cost = INFINITY;

    for i in 0..entities.len() {
        match entities[i].behaviour {

            // Chase humans and cops
            Behaviour::Cop { .. } | Behaviour::Human => {
                match find_path(my_pos, entities[i].position, buildings, building_outlines) {
                    None => (),
                    Some(path) => if path.cost < min_cost {
                        min_cost = path.cost;
                        min_path = Some(path);
                    }
                }
            }

            // Skip everything else
            _ => ()
        }
    }

    match min_path {
        None => (),
        Some(path) => {
            match path.to_vec().get(1) {
                None => (),
                Some(&node) => {
                    let delta = node - my_pos;
                    entities[index].accelerate_along_vector(delta, args.dt);
                }
            }
        }
    }
}

fn simulate_human(args: &UpdateArgs, entities: &mut Vec<Entity>, index: usize) {
    let my_pos = entities[index].position;

    let mut min_delta = Vector2::zero();
    let mut min_distance_sqr = INFINITY;

    for i in 0..entities.len() {
        match entities[i].behaviour {

            // Run from zombies
            Behaviour::Zombie => {
                let delta = entities[i].position - my_pos;
                let distance_sqr = delta.length_squared();
                if distance_sqr < min_distance_sqr {
                    min_delta = delta;
                    min_distance_sqr = distance_sqr;
                }
            }

            // Skip everything else
            _ => ()
        }
    }

    if min_distance_sqr < INFINITY {
        // Accelerate away from the nearest zombie
        entities[index].accelerate_along_vector(-min_delta, args.dt);
    }
}
