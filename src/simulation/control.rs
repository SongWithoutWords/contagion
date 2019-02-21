
use crate::core::vector::*;
use crate::core::scalar::Scalar;
use crate::core::matrix::*;

use glium_sdl2::SDL2Facade;

use super::state::*;

pub fn update_selected(_action_type: u32, state: &mut State, window: &SDL2Facade, camera_frame: Mat4, x_mouse: i32, y_mouse: i32) {
    state.is_selected = vec![false; state.entities.len()];
    let m_pos = &mut Vector2{ x : x_mouse as f64, y : y_mouse as f64};
    translate_mouse_to_camera(m_pos, window.window().size());
    translate_camera_to_world(m_pos, camera_frame);

    for i in 0..state.entities.len() {
        let entity = &mut state.entities[i];
        match entity.behaviour {
            Behaviour::Cop {..} => {
                let x_pos: Scalar = entity.position.x;
                let y_pos: Scalar = entity.position.y;
                if m_pos.x <= x_pos + 0.5 && m_pos.x >= x_pos - 0.5
                    && m_pos.y <= y_pos + 0.5 && m_pos.y >= y_pos - 0.5 {
                    state.is_selected[i] = true;
                }
            }
            _ => ()
        }
    }
}

// Issue an order to selected police
pub fn issue_police_order(order: PoliceOrder, state: &mut State, window: &SDL2Facade, camera_frame: Mat4, x_mouse: i32, y_mouse: i32) {
    match order {
        PoliceOrder::Move => {
            let m_pos = &mut Vector2{ x : x_mouse as f64, y : y_mouse as f64};
            translate_mouse_to_camera(m_pos, window.window().size());
            translate_camera_to_world(m_pos, camera_frame);
            for i in 0..state.is_selected.len() {
                if state.is_selected[i] {
                    match state.entities[i].behaviour {
                        Behaviour::Cop {ref mut state, ..} => {
                            *state = CopState::Moving{ waypoint: *m_pos }
                        }
                        _ => ()
                    }
                }
            }
        }
        _=>()
    }
}

pub fn translate_mouse_to_camera(vec: &mut Vector2, window_size: (u32, u32)) {
    vec.x = vec.x / window_size.0 as f64 * 2.0 - 1.0;
    vec.y = -(vec.y / window_size.1 as f64 * 2.0 - 1.0);
}

pub fn translate_camera_to_world(vec: &mut Vector2, matrix: Mat4) {
    let inverse_matrix = matrix.inverse_matrix4();
    let temp_vec2 = Vector2{x: vec.x, y: vec.y};
    let new_vec2 = inverse_matrix.multiply_vec2(temp_vec2);
    vec.x = new_vec2.x;
    vec.y = new_vec2.y;
}

pub enum PoliceOrder {
    Move,
    Shoot
}


