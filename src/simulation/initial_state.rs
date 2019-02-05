extern crate rand;

use rand::Rng;
use crate::core::vector::*;
use crate::core::scalar::*;
use super::state::*;

pub fn initial_state(count: u32) -> State {
    let human_count: u32 = (count as f32 * 0.8) as u32;
    let cop_count: u32 = (count as f32 * 0.18) as u32;
    let zombie_count = count - (human_count + cop_count);
    let mut state = State { entities: vec!(), is_selected: vec!(), projectiles: vec!()};

    let entities = &mut state.entities;

    let mut rng = rand::thread_rng();
    for i in 0..count {
        // TODO: need to optimize this later with housing units and two entities shouldn't be placed on same tile
        let x = rng.gen_range(0.0, 25 as Scalar);
        let y = rng.gen_range(0.0, 25 as Scalar);
        let facing_angle = rng.gen_range(0.0, 1 as Scalar);
        let position = vector2(x, y);
        let velocity = Vector2::zero();
        // spawn 80% humans
        let behaviour = if i < human_count {
            Behaviour::Human
        } // spawn 18% cops
        else if  i >= human_count && i < (count - zombie_count) {
            Behaviour::Cop {
                rounds_in_magazine: COP_MAGAZINE_CAPACITY,
                state: CopState::Idle
            }
        }
        // spawn rest zombie
        else {
            Behaviour::Zombie
        };
        entities.push(Entity { position, velocity, facing_angle, behaviour });
    }
    state
}
