use crate::core::vector::*;
use crate::core::scalar::*;
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

    let mut entities = &mut state.entities;

    // Apply individual behaviours
    for i in 0..entities.len() {
        match entities[i].behaviour {
            Behaviour::Cop {waypoint} =>
            // TODO: Shoot zombies
                simulate_cop(args, &mut entities, i, waypoint),
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

fn simulate_cop(args: &UpdateArgs, entities: &mut Vec<Entity>, index: usize, final_dest: Option<Vector2>) {
    let my_pos = entities[index].position;

    let mut min_delta = Vector2{x: 1.0, y: 1.0};

    if let Some(ref m) = final_dest {
        println!("{}, {}", m.x, m.y);
    }

    // entities[index].velocity += args.dt * min_delta;

//    if (final_dest == None) {
//        // do nothing
//    } else if (my_pos.x == final_dest.x && my_pos.y == final_dest.y) {
//        // todo: create a range of episilon to stop the entity
//        // stop the unit
//        final_dest == None;
//    } else {
//        // todo: move towards final destination
//        entities[index].velocity += args.dt * min_delta;
//    }
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
