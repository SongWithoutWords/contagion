use rand::*;
use rand_xorshift::XorShiftRng;
use std::collections::HashSet;
use crate::core::vector::*;
use crate::core::scalar::*;
use crate::core::geo::polygon::*;
use super::state::*;

const PORTION_OF_ENTITIES_ZOMBIE: f32 = 0.2;
const PORTION_OF_ENTITIES_COP: f32 = 0.05;

pub fn initial_state(entity_count: u32, random_seed: u32) -> State {
    let entity_count_f32 = entity_count as f32;
    let zombie_count: u32 = ((entity_count_f32 * PORTION_OF_ENTITIES_ZOMBIE) as u32).min(1);
    let cop_count: u32 = ((entity_count_f32 * PORTION_OF_ENTITIES_COP) as u32).min(1);
    let human_count = (entity_count - (zombie_count + cop_count)).min(1);
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

   buildings.push(Polygon(vec![
       Vector2 { x: -5.0, y: -5.0 },
       Vector2 { x: -5.0, y: 5.0 },
       Vector2 { x: 5.0, y: 5.0 },
       Vector2 { x: 5.0, y: -5.0 }]));

    for i in 0..entity_count {
        // TODO: need to optimize this later with housing units and two entities shouldn't be placed on same tile
        let x = -7.0;//state.rng.gen_range(0.0, 25 as Scalar);
        let y = 0.0;//state.rng.gen_range(0.0, 25 as Scalar);
        let facing_angle = state.rng.gen_range(0.0, 1 as Scalar);
        let position = vector2(x, y);
        let velocity = Vector2::zero();

        let behaviour = if i < cop_count {
            Behaviour::Cop {
                rounds_in_magazine: COP_MAGAZINE_CAPACITY,
                state: CopState::Idle
            }
        }
        else if i < cop_count + zombie_count {
            Behaviour::Zombie
        }
        else {
            Behaviour::Human
        };
        entities.push(Entity { position, velocity, facing_angle, behaviour });
    }

    // Generate outlines around all buildings for building A* pathfinding graphs
    for i in 0..buildings.len() {
        let mut outlines = vec!();
        let norms = buildings[i].normals();

        for j in 0..buildings[i].num_sides() {
            let norm_sum = norms[j] + norms[if j < 1 { buildings[i].num_sides() - 1 } else { j - 1 }];
            let offset = norm_sum * ENTITY_RADIUS * 1.1;
            outlines.push(offset + buildings[i].get(j));
        }

        building_outlines.push(Polygon(outlines));
    }

    state
}
