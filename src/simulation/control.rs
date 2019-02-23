
use crate::core::vector::*;
use crate::core::scalar::Scalar;
use crate::core::matrix::*;
use crate::core::geo::intersect::rectangle_point::*;

use glium_sdl2::SDL2Facade;
use sdl2::event::Event;
use std::time::Instant;
use sdl2::mouse::MouseButton;

use super::state::*;

pub struct Control {
    pub mouse_drag: bool,
    pub drag_start_mouse_coord: Vector2,
    pub drag_vertex_start: Vector2,
    pub drag_vertex_end: Vector2,
    pub last_click_time: Instant
}

impl Control {
    pub fn new() -> Control {
        Control {
            mouse_drag: false,
            drag_start_mouse_coord: Vector2::zero(),
            drag_vertex_start: Vector2::zero(),
            drag_vertex_end: Vector2::zero(),
            last_click_time: Instant::now()
        }
    }

    pub fn click_select(&mut self, state: &mut State, window: &SDL2Facade, camera_frame: Mat4, mouse_pos: Vector2) {
        state.selection.clear();
        let m_pos = &mut Vector2{ x : mouse_pos.x, y : mouse_pos.y};
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
                        state.selection.insert(i);
                    }
                }
                _ => ()
            }
        }
    }

    pub fn double_click_select(&mut self, state: &mut State, camera_frame: Mat4) {
        state.selection.clear();
        for i in 0..state.entities.len() {
            let entity = &mut state.entities[i];
            match entity.behaviour {
                Behaviour::Cop {..} => {
                    let entity_pos = &mut Vector2{ x: entity.position.x, y: entity.position.y };
                    translate_world_to_camera(entity_pos, camera_frame);
                    if entity_pos.x <= 1.0 && entity_pos.x >= -1.0
                        && entity_pos.y <= 1.0 && entity_pos.y >= -1.0 {
                        state.selection.insert(i);
                    }
                }
                _ => ()
            }
        }
    }

    pub fn drag_select(&mut self, state: &mut State, window: &SDL2Facade, camera_frame: Mat4, mouse_end: Vector2) {
        state.selection.clear();
        let m_start_pos = &mut Vector2{ x : self.drag_start_mouse_coord.x, y : self.drag_start_mouse_coord.y};
        let m_end_pos = &mut Vector2{ x : mouse_end.x, y : mouse_end.y};
        translate_mouse_to_camera(m_start_pos, window.window().size());
        translate_mouse_to_camera(m_end_pos, window.window().size());

        self.drag_vertex_start.x = m_start_pos.x;
        self.drag_vertex_start.y = m_start_pos.y;
        self.drag_vertex_end.x = m_end_pos.x;
        self.drag_vertex_end.y = m_end_pos.y;

        translate_camera_to_world(m_start_pos, camera_frame);
        translate_camera_to_world(m_end_pos, camera_frame);

        for i in 0..state.entities.len() {
            let entity = &mut state.entities[i];
            match entity.behaviour {
                Behaviour::Cop {..} => {
                    let entity_pos = entity.position;
                    if check_bounding_box(*m_start_pos, *m_end_pos, entity_pos) {
                        state.selection.insert(i);
                    }
                }
                _ => ()
            }
        }
    }

    // Issue an order to selected police
    pub fn issue_police_order(&mut self, order: PoliceOrder, state: &mut State, window: &SDL2Facade, camera_frame: Mat4, mouse_pos: Vector2) {
        match order {
            PoliceOrder::Move => {
                let m_pos = &mut Vector2{ x: mouse_pos.x, y: mouse_pos.y };
                translate_mouse_to_camera(m_pos, window.window().size());
                translate_camera_to_world(m_pos, camera_frame);
                for i in &state.selection {
                    match state.entities[*i].behaviour {
                        Behaviour::Cop { ref mut state, .. } => {
                            *state = CopState::Moving { waypoint: *m_pos }
                        }
                        _ => ()
                    }
                }
            }
            _=>()
        }
    }

    pub fn update_drag_start(&mut self, new_drag_start: Vector2, window: &SDL2Facade) {
        let drag_start_proj = &mut Vector2{ x : new_drag_start.x, y : new_drag_start.y};
        self.drag_start_mouse_coord.x = new_drag_start.x;
        self.drag_start_mouse_coord.y = new_drag_start.y;
        translate_mouse_to_camera(drag_start_proj, window.window().size());

        self.drag_vertex_start.x = drag_start_proj.x;
        self.drag_vertex_start.y = drag_start_proj.y;
    }

    pub fn update_drag_end(&mut self, new_drag_end: Vector2, window: &SDL2Facade) {
        let drag_end_proj = &mut Vector2{ x : new_drag_end.x, y : new_drag_end.y};
        translate_mouse_to_camera(drag_end_proj, window.window().size());

        self.drag_vertex_end.x = drag_end_proj.x;
        self.drag_vertex_end.y = drag_end_proj.y;
    }

    pub fn handle_event(&mut self, event: Event, window: &SDL2Facade, camera_frame: Mat4, state: &mut State) {
        match event {
            Event::MouseButtonDown { timestamp: _, window_id: _, which: _, mouse_btn: _, x, y } => {
                self.mouse_drag = true;
                let mouse_pos = Vector2 { x: x as f64, y: y as f64 };
                self.update_drag_start(mouse_pos, &window);
                self.update_drag_end(mouse_pos, &window);
            }
            Event::MouseMotion {
                timestamp: _,
                window_id: _,
                which: _,
                mousestate: _,
                x,
                y,
                xrel: _,
                yrel: _, } => {
                if self.mouse_drag {
                    let mouse_pos = Vector2 { x: x as f64, y: y as f64 };
                    self.update_drag_end(mouse_pos, &window);
                }
            }
            Event::MouseButtonUp { timestamp: _, window_id: _, which: _, mouse_btn, x, y } => {
                self.mouse_drag = false;
                let mouse_pos = Vector2 { x: x as f64, y: y as f64 };

                match mouse_btn {
                    MouseButton::Left { .. } => {
                        // Select one police if delta of drag is too small, else select all police in drag
                        let delta = 1.0;

                        if (mouse_pos.x - self.drag_start_mouse_coord.x).abs() <= delta && (mouse_pos.y - self.drag_start_mouse_coord.y).abs() <= delta {
                            let current_time = Instant::now();
                            let delta_millisecond = 300;
                            let duration = current_time.duration_since(self.last_click_time);
                            if duration.as_secs() == 0 && duration.subsec_millis() < delta_millisecond {
                                self.double_click_select(state, camera_frame);
                            } else {
                                self.click_select(state, &window, camera_frame, mouse_pos);
                            }
                            self.last_click_time = current_time;
                        } else {
                            self.drag_select(state, &window, camera_frame, mouse_pos);
                        }
                    }
                    MouseButton::Right { .. } => {
                        self.issue_police_order(PoliceOrder::Move, state, &window, camera_frame, mouse_pos);
                    }
                    _ => ()
                }
            }
            _ => ()
        }
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

pub fn translate_world_to_camera(vec: &mut Vector2, matrix: Mat4) {
    let temp_vec2 = Vector2{x: vec.x, y: vec.y};
    let new_vec2 = matrix.multiply_vec2(temp_vec2);
    vec.x = new_vec2.x;
    vec.y = new_vec2.y;
}

pub enum PoliceOrder {
    Move,
    Shoot
}


