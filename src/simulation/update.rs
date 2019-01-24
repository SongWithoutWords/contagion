use crate::core::vector::*;
use crate::core::scalar::*;
use super::state::*;

pub struct UpdateArgs {
    dt: Scalar
}

pub fn update(args: &UpdateArgs, state: &mut State) {

    let mut entities = &mut state.entities;

    // Apply individual behaviours
    for i in 0..entities.len() {
        match entities[i].behaviour {
            Behaviour::Cop =>
            // TODO: Shoot zombies
                (),
            Behaviour::Dead =>
            // Do nothing
                (),
            Behaviour::Human =>
            // Run from zombies!
                simulate_human(args, &mut entities, i),
            Behaviour::Zombie =>
            // Chase humans and cops!
                simulate_zombie(args, &mut entities, i)
        }
    }
}

fn simulate_zombie(args: &UpdateArgs, entities: &mut Vec<Entity>, index: usize) {

    let my_pos = entities[index].position;

    let mut min_delta = Vector2::zero();
    let mut min_distance_sqr = INFINITY;

    for i in 0..entities.len() {
        match entities[i].behaviour {

            // Chase humans and cops
            Behaviour::Cop | Behaviour::Human => {
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
