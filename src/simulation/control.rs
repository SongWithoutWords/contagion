
use crate::core::scalar::Scalar;
use super::state::*;

pub fn update_selected(action_type: u32, state: &mut State, x_mouse: i32, y_mouse: i32) {
    state.is_selected = vec![false; state.entities.len()];
    let x_mouse = x_mouse as f64;
    let y_mouse = y_mouse as f64;
    println!("x_mouse {:?}, y_mouse {:?}", x_mouse, y_mouse);

    for i in 0..state.entities.len() {
        let entity = &mut state.entities[i];
        match entity.behaviour {
            Behaviour::Cop => {
                let x_pos: Scalar = entity.position.x;
                let y_pos: Scalar = entity.position.y;
                println!("x_pos {:?}, y_pos {:?}", x_pos, y_pos);
//                if (x_mouse <= x_pos + 0.5 && x_mouse >= x_pos - 0.5
//                    && y_mouse <= y_pos + 0.5 && y_mouse >= y_pos - 0.5) {
//                    state.is_selected[i] = true;
//                }
                if (x_mouse <= 520.0 && x_mouse >= 480.0
                    && y_mouse <= 400.0 && y_mouse >= 370.0) {
                    state.is_selected[i] = true;
                }
            }
            _ => ()
        }
    }
}

// Issue an order to selected police
pub fn issue_police_order(order: PoliceOrder, state: &mut State) {
    match order {
        PoliceOrder::Move => {
            // TODO: set police destination or something cancer!
        }
        _=>()
    }
}

pub enum PoliceOrder {
    Move,
    Shoot
}


