use crate::core::geo::circle::*;
use crate::core::geo::segment2::*;
use crate::core::geo::intersect::segment_circle::*;
use crate::core::scalar::*;
use crate::core::vector::*;
use super::state::*;

pub struct UpdateArgs {
    pub dt: Scalar
}

pub enum SoundEffect {
    Gunshot,
    Reload,
    PersonInfected,
    ZombieDeath,
}

pub fn update(args: &UpdateArgs, state: &mut State) -> Vec<SoundEffect> {

    // Apply individual behaviours
    for i in 0..state.entities.len() {
        match &state.entities[i].behaviour {
            b @ Behaviour::Cop{..} => {
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
                simulate_zombie(args, &mut state.entities, i)
        }
    }

    const DOUBLE_ENTITY_RADIUS_SQUARED: f64 = 4.0 * ENTITY_RADIUS * ENTITY_RADIUS;

    // Check for collisions
    for i in 0..state.entities.len() {
        let p1 = state.entities[i].position;

        for j in (i+1)..state.entities.len() {
            let p2 = state.entities[j].position;

            let delta = p2 - p1;

            let delta_length_squared = delta.length_squared();

            if delta_length_squared < DOUBLE_ENTITY_RADIUS_SQUARED {
                handle_collision(args, &mut state.entities, i, j, &delta, delta_length_squared);
            }
        }
    }

    // Apply acceleration
    for e in &mut state.entities {
        let displacement = args.dt * e.velocity;
        e.position += displacement;
        e.velocity -= 0.5 * displacement;
    }

    // Remove motionless projectiles
    state.projectiles.retain(
        |p| p.velocity.length_squared() > MIN_PROJECTILE_SPEED
    );

    // Simulate projectiles
    for projectile in &mut state.projectiles {
        let displacement = args.dt * projectile.velocity;

        let segment = Segment2 { p1: projectile.position, p2: projectile.position + displacement };

        let mut first_intersect_time_and_index = None;
        for i in 0..state.entities.len() {

            let entity = &state.entities[i];

            if let Behaviour::Dead = entity.behaviour {
                // Dead entities don't collide with bullets
                continue;
            }

            let circle = Circle { center: entity.position, radius: ENTITY_RADIUS };

            let this_min_intersection = segment_circle_min_positive_intersect_time(&segment, &circle);

            match (first_intersect_time_and_index, this_min_intersection) {
                (None, Some(this)) => {
                    first_intersect_time_and_index = Some((this, i))
                },
                (Some((min, _)), Some(this)) if this < min => {
                    first_intersect_time_and_index = Some((this, i))
                },
                _ => ()
            }
        }

        match first_intersect_time_and_index {
            None => (),
            Some((_, i)) => {
                state.entities[i].behaviour = Behaviour::Dead;
                projectile.velocity = Vector2::zero();
            }
        }

        projectile.position = segment.p2;
    }

    vec!()
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

        (Behaviour::Human, Behaviour::Zombie) => entities[i].behaviour = Behaviour::Zombie,
        (Behaviour::Zombie, Behaviour::Human) => entities[j].behaviour = Behaviour::Zombie,

        (Behaviour::Cop {..}, Behaviour::Zombie) => entities[i].behaviour = Behaviour::Zombie,
        (Behaviour::Zombie, Behaviour::Cop {..}) => entities[j].behaviour = Behaviour::Zombie,

        _ => ()
    }

    // Force entities apart that are overlapping
    let velocity_change = *delta * (args.dt / delta_length_squared);
    entities[i].velocity -= velocity_change;
    entities[j].velocity += velocity_change;
}

fn update_cop(
    args: &UpdateArgs,
    sim_state: &mut State,
    index: usize,
    behaviour: Behaviour) -> Behaviour {

    let entities = &mut sim_state.entities;

    match behaviour {

        Behaviour::Cop { rounds_in_magazine, state} => {
            match state {
                CopState::Aiming { mut aim_time_remaining, target_index} => {

                    // TODO: check if we can still see the target, and stop aiming if not

                    aim_time_remaining -= args.dt;
                    if aim_time_remaining > 0.0 {
                        // Taking aim, do nothing
                        Behaviour::Cop{
                            rounds_in_magazine: rounds_in_magazine,
                            state: CopState::Aiming{aim_time_remaining, target_index: target_index}
                        }
                    }
                    else {
                        // Finished aiming, take the shot
                        let my_pos:Vector2 = entities[index].position;
                        let target_pos = entities[target_index].position;
                        let delta_normal = (target_pos - my_pos).normalize();

                        // Fire at the taget
                        sim_state.projectiles.push(
                            Projectile {
                                // Spawn outside of the entity - don't want to shoot the entity itself
                                position: entities[index].position + 1.125 * ENTITY_RADIUS * delta_normal,
                                velocity: BULLET_SPEED * delta_normal
                            });
                        Behaviour::Cop{
                            rounds_in_magazine: rounds_in_magazine - 1,
                            state: CopState::Aiming{aim_time_remaining, target_index: target_index}
                        }
                    }
                },
                CopState::Moving { waypoint } => {
                    let delta = waypoint - entities[index].position;
                    if delta.length_squared() < COP_MIN_DISTANCE_FROM_WAYPOINT_SQUARED {
                        Behaviour::Cop{
                            rounds_in_magazine: rounds_in_magazine,
                            state: CopState::Idle
                        }
                    } else {
                        // todo: move towards the waypoint
                        entities[index].velocity += delta.normalize_to(args.dt);
                        Behaviour::Cop{
                            rounds_in_magazine: rounds_in_magazine,
                            state: CopState::Moving{ waypoint }
                        }
                    }
                },
                CopState::Reloading { mut reload_time_remaining } => {
                    reload_time_remaining -= args.dt;
                    if reload_time_remaining > 0.0 {
                        // Reloading, do nothing
                        Behaviour::Cop{
                            rounds_in_magazine: rounds_in_magazine,
                            state: CopState::Reloading{reload_time_remaining: reload_time_remaining}
                        }
                    }
                    else {
                        // Finished reloading, replenish rounds
                        Behaviour::Cop{
                            rounds_in_magazine: COP_MAGAZINE_CAPACITY,
                            state: CopState::Idle
                        }
                    }
                },
                CopState::Idle => {
                    // Reload if you don't have ammo
                    if rounds_in_magazine <= 0 {
                        Behaviour::Cop{
                            rounds_in_magazine: rounds_in_magazine,
                            state: CopState::Reloading{reload_time_remaining: COP_RELOAD_COOLDOWN}
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
                            Behaviour::Cop {
                                rounds_in_magazine: rounds_in_magazine,
                                state: CopState::Aiming {
                                    aim_time_remaining: COP_AIM_COOLDOWN,
                                    target_index: min_index
                                }
                            }
                        }
                        else {
                            // Remain in idle state
                            behaviour
                        }
                    }
                }
            }
        },
        _ => panic!("Entity at index should be a cop!")
    }
}

fn simulate_zombie(args: &UpdateArgs, entities: &mut Vec<Entity>, index: usize) {

    let my_pos = entities[index].position;

    let mut min_delta = Vector2::zero();
    let mut min_distance_sqr = INFINITY;

    for i in 0..entities.len() {
        match entities[i].behaviour {

            // Chase humans and cops
            Behaviour::Cop{..} | Behaviour::Human => {
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
        // Accelerate towards the nearest target
        entities[index].velocity += args.dt * min_delta.normalize();
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
        entities[index].velocity -= min_delta.normalize_to(args.dt);
    }
}
