use crate::core::vector::*;
use crate::core::scalar::*;
use crate::simulation::state::*;

pub fn display(state: &State) {

    for entity in &state.entities {

        match entity.behaviour {

            Behaviour::Cop =>
            // TODO: Draw a cop
                (),

            Behaviour::Dead =>
            // TODO: Draw a corpse
            // or if that's not what we want for the tone of the game, then don't!
                (),

            Behaviour::Human =>
            // TODO: Draw a civilian
                (),

            Behaviour::Zombie =>
            // TODO: Draw a zombie
                (),
        }
    }
}
