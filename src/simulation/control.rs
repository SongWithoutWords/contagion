
use crate::constants::presentation::*;
use crate::core::vector::*;
use crate::core::scalar::Scalar;
use glium_sdl2::SDL2Facade;
use super::state::*;

pub fn update_selected(action_type: u32, state: &mut State, window: &SDL2Facade, x_mouse: i32, y_mouse: i32) {
    state.is_selected = vec![false; state.entities.len()];
    let m_pos = &mut Vector2{ x : x_mouse as f64, y : y_mouse as f64};
    translate_mouse_to_camera(m_pos, window.window().size());
    translate_camera_to_world(m_pos);
    println!("x_mouse {:?}, y_mouse {:?}", m_pos.x, m_pos.y);

    for i in 0..state.entities.len() {
        let entity = &mut state.entities[i];
        match entity.behaviour {
            Behaviour::Cop {..} => {
                let x_pos: Scalar = entity.position.x;
                let y_pos: Scalar = entity.position.y;
                println!("x_pos {:?}, y_pos {:?}", x_pos, y_pos);
//                if (x_mouse <= x_pos + 0.5 && x_mouse >= x_pos - 0.5
//                    && y_mouse <= y_pos + 0.5 && y_mouse >= y_pos - 0.5) {
//                    state.is_selected[i] = true;
//                }
            }
            _ => ()
        }
    }
}

// Issue an order to selected police
pub fn issue_police_order(order: PoliceOrder, state: &mut State, x_mouse: i32, y_mouse: i32) {
    match order {
        PoliceOrder::Move => {
            let x_mouse = x_mouse as f64;
            let y_mouse = y_mouse as f64;
            for i in 0..state.is_selected.len() {
                if (state.is_selected[i] == true) {
                    match state.entities[i].behaviour {
                        Behaviour::Cop {mut waypoint} => {
                            // todo: get this ref mut way_point thing to work ??
                            // todo: dynamic x_xmouse, y_mouse positions
                            waypoint = Some(Vector2{x: 1.0, y:1.0});
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

pub fn translate_camera_to_world(vec: &mut Vector2) {

}

pub enum PoliceOrder {
    Move,
    Shoot
}


