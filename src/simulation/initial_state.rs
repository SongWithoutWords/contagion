extern crate rand;

use rand::Rng;
use crate::core::vector::*;
use crate::core::scalar::*;
use crate::constants::presentation::*;

use super::state::*;

pub fn initial_state(count: u32) -> State {

    let mut state = State { entities: vec!()};

    let entities = &mut state.entities;

    for _i in 0..count {
        // TODO: need to optimize this later with housing units and two entities shouldn't be placed on same tile
        let mut rng = rand::thread_rng();
        let x: Scalar = rng.gen_range(0.0f64, WINDOW_W as f64);
        let y: Scalar = rng.gen_range(0.0f64, WINDOW_H as f64);
        let position = Vector2{x: x, y: y};
        let velocity = Vector2::zero();
        entities.push(generate_human(position, velocity));
    }
    state
}

pub fn generate_human(position: Vector2, velocity: Vector2) -> Entity {
    let human = Entity{position, velocity, behaviour: Behaviour::Human};
    return human;
}
