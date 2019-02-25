use rand::*;
use rand_xorshift::XorShiftRng;
use std::collections::HashSet;
use crate::core::vector::*;
use crate::core::scalar::*;
use crate::core::geo::polygon::*;
use super::state::*;

pub fn initial_state(entity_count: u32, random_seed: u32) -> State {
    let human_count: u32 = (entity_count as f32 * 0.8) as u32;
    let cop_count: u32 = (entity_count as f32 * 0.18) as u32;
    let zombie_count = entity_count - (human_count + cop_count);
    let mut state = State {
        entities: vec!(),
        buildings: vec!(),
        building_outlines: vec!(),
        selection: HashSet::new(),
        projectiles: vec!(),
        rng: XorShiftRng::seed_from_u64(random_seed as u64)
    };

    let entities = &mut state.entities;
    let buildings = &mut state.buildings;
    let building_outlines = &mut state.building_outlines;

   buildings.push(Polygon(vec![Vector2 { x: 7.5, y: 7.5 }, Vector2 { x: 15.0, y: 7.5 },
                               Vector2 { x: 15.0, y: 15.0 }, Vector2 { x: 7.5, y: 15.0 }]));

    // let mut rng = rand::thread_rng();
//    for i in 0..entity_count {
        // TODO: need to optimize this later with housing units and two entities shouldn't be placed on same tile
        let x = 0.0;//state.rng.gen_range(0.0, 25 as Scalar);
        let y = 0.0;//state.rng.gen_range(0.0, 25 as Scalar);
        let facing_angle = state.rng.gen_range(0.0, 1 as Scalar);
        let position = vector2(x, y);
        let velocity = Vector2::zero();
//        // spawn 80% humans
//        let behaviour = if i < human_count {
//            Behaviour::Human
//        } // spawn 18% cops
//        else if  i >= human_count && i < (entity_count - zombie_count) {
//            Behaviour::Cop {
//                rounds_in_magazine: COP_MAGAZINE_CAPACITY,
//                state: CopState::Idle
//            }
//        }
//        // spawn rest zombie
//        else {
//            Behaviour::Zombie
//        };
        let behaviour = Behaviour::Cop {
            rounds_in_magazine: COP_MAGAZINE_CAPACITY,
            state: CopState::Idle
        };
        entities.push(Entity { position, velocity, facing_angle, behaviour });
//    }

    // Generate outlines around all buildings for building A* pathfinding graphs
    for i in 0..buildings.len() {
        let mut outlines = vec!();
        let norms = buildings[i].normals();

        for j in 0..buildings[i].num_sides() {
            let norm_sum = norms[j] + norms[if j < 1 { buildings[i].num_sides() - 1 } else { j - 1 }];
            outlines.push(norm_sum * ENTITY_RADIUS * 1.1);
        }

        building_outlines.push(Polygon(outlines));
    }

    state
}
