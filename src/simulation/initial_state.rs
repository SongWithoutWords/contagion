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
    for _i in 0..count {
        // TODO: need to optimize this later with housing units and two entities shouldn't be placed on same tile
        let x: Scalar = rng.gen_range(0.0f64, 25 as f64);
        let y: Scalar = rng.gen_range(0.0f64, 25 as f64);
        let position = vector2(x, y);
        let velocity = Vector2::zero();
        // spawn 80% humans
        if _i < human_count {
            entities.push(Entity { position, velocity, behaviour: Behaviour::Human });
        } // spawn 18% cops
        else if  _i >= human_count && _i < (count - zombie_count) {
            entities.push( Entity {
                position,
                velocity,
                behaviour: Behaviour::Cop {
                    rounds_in_magazine: COP_MAGAZINE_CAPACITY,
                    state: CopState::Idle
                }
            });
        }
        // spawn rest zombie
        else {
            entities.push(Entity { position, velocity, behaviour: Behaviour::Zombie });
        }
    }
    state
}
