use crate::core::scalar::*;
use crate::core::vector::*;
use crate::core::matrix::*;
use glium_sdl2::SDL2Facade;
use sdl2::keyboard;
use sdl2::video::FullscreenType::{True};

#[derive(Clone)]
pub struct Camera {
    position: Vector2,
    velocity: Vector2,
    zoom: Vector2,
    initial_mouse_pos: Vector2,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: Vector2::zero(),
            velocity: Vector2::zero(),
            zoom: Vector2 { x: 0.09 as f64, y: 0.09 as f64 },  // set initial zoom level
            initial_mouse_pos: Vector2 { x: 0.0 as f64, y: 0.0 as f64 },
        }
    }

    const DRAG_FACTOR: Scalar = 8.0;
    const ACCELERATION_FACTOR: Scalar = 12.0;

    pub fn update(
        &mut self,
        ks: &sdl2::keyboard::KeyboardState,
        ms: &sdl2::mouse::MouseState,
        window: &mut SDL2Facade,
        delta_time: Scalar) {

        const LEFT_EDGE_SCREEN: Scalar = -1.0;
        const RIGHT_EDGE_SCREEN: Scalar = 0.99;
        const TOP_EDGE_SCREEN: Scalar = 1.0;
        const BOTTOM_EDGE_SCREEN: Scalar = -0.99;

        let mut acceleration = Vector2 {
            x: self.key_pressed(ks, keyboard::Scancode::D) - self.key_pressed(ks, keyboard::Scancode::A),
            y: self.key_pressed(ks, keyboard::Scancode::W) - self.key_pressed(ks, keyboard::Scancode::S),
        };

        // Allow mouse cursor to move camera when against screen edges in fullscreen
        if window.window().fullscreen_state() == True {
            const LEFT_BOUND: Scalar = -25.0;
            const RIGHT_BOUND: Scalar = 115.0;
            const TOP_BOUND: Scalar = 115.0;
            const BOTTOM_BOUND: Scalar = -25.0;

            let left_corner_bound = vector2(LEFT_BOUND * self.zoom.x, BOTTOM_BOUND * self.zoom.y);
            let right_corner_bound = vector2(RIGHT_BOUND * self.zoom.x, TOP_BOUND * self.zoom.y);

            let mouse_pos = &mut vector2(ms.x() as f64, ms.y() as f64);
            translate_to_camera_coord(mouse_pos, window.window().size());

            if mouse_pos.x == LEFT_EDGE_SCREEN && self.position.x > left_corner_bound.x {
                acceleration.x = -1.0;
            } else if mouse_pos.x >= RIGHT_EDGE_SCREEN && self.position.x < right_corner_bound.x {
                acceleration.x = 1.0;
            }

            if mouse_pos.y <= BOTTOM_EDGE_SCREEN && self.position.y > left_corner_bound.y {
                acceleration.y = -1.0;
            } else if mouse_pos.y == TOP_EDGE_SCREEN && self.position.y < right_corner_bound.y {
                acceleration.y = 1.0;
            }
        }


        self.velocity += delta_time * Self::ACCELERATION_FACTOR * acceleration;
        self.position += delta_time * self.velocity;
        self.velocity -= delta_time * Self::DRAG_FACTOR * self.velocity;
    }

    fn key_pressed(&mut self, ks: &sdl2::keyboard::KeyboardState, s: keyboard::Scancode) -> Scalar {
        const LEFT_BOUND: Scalar = -25.0;
        const RIGHT_BOUND: Scalar = 115.0;
        const TOP_BOUND: Scalar = 115.0;
        const BOTTOM_BOUND: Scalar = -25.0;

        let left_corner_bound = vector2(LEFT_BOUND * self.zoom.x, BOTTOM_BOUND * self.zoom.y);
        let right_corner_bound = vector2(RIGHT_BOUND * self.zoom.x, TOP_BOUND * self.zoom.y);

        if ks.is_scancode_pressed(s) {
            if s == keyboard::Scancode::A && self.position.x < left_corner_bound.x {
                return 0.0;
            } else if  s == keyboard::Scancode::D && self.position.x > right_corner_bound.x {
                return 0.0;
            }

            if s == keyboard::Scancode::S && self.position.y < left_corner_bound.y {
                return 0.0;
            } else if  s == keyboard::Scancode::W && self.position.y > right_corner_bound.y {
                return 0.0;
            }
            else {
                return 1.0;
            }
        } else {
            return 0.0;
        }
    }

    // Camera frame in world coordinates
    pub fn compute_matrix(&self) -> Mat4 {
        (Mat4 {
            i: Vector4 { x: self.zoom.x, y: 0.0, z: 0.0, w: 0.0 },
            j: Vector4 { x: 0.0, y: self.zoom.y, z: 0.0, w: 0.0 },
            k: Vector4 { x: 0.0, y: 0.0, z: 1.0, w: 0.0 },
            w: Vector4 { x: -self.position.x, y: -self.position.y, z: 0.0, w: 1.0 },
        })
    }

    pub fn set_initial_mouse_pos(&mut self, mouse_pos_x: i32, mouse_pos_y: i32) {
        let initial_mouse_pos = &mut Vector2 { x: mouse_pos_x as f64, y: mouse_pos_y as f64 };
        self.initial_mouse_pos = *initial_mouse_pos;
    }

    pub fn camera_pan(&mut self, mouse_pos_x: i32, mouse_pos_y: i32,
                      ms: &sdl2::mouse::MouseState,
                      window: &SDL2Facade,
                      camera_frame: Mat4) {

        const PAN_SPEED: Scalar = 0.3;

        let mouse_pos = &mut vector2(mouse_pos_x as f64, mouse_pos_y as f64);
        translate_to_camera_coord(mouse_pos, window.window().size());
        translate_camera_to_world_coord(mouse_pos, camera_frame);

        let initial_mouse_pos: &mut Vector2 = &mut Vector2 {x: self.initial_mouse_pos.x, y: self.initial_mouse_pos.y};
        translate_to_camera_coord(initial_mouse_pos, window.window().size());
        translate_camera_to_world_coord(initial_mouse_pos, camera_frame);

        if ms.middle() {
            let delta_mouse = vector2((initial_mouse_pos.x - mouse_pos.x) * self.zoom.x, (initial_mouse_pos.y - mouse_pos.y) * self.zoom.y);

            self.position.x += delta_mouse.x * PAN_SPEED;
            self.position.y += delta_mouse.y * PAN_SPEED;
        }
    }

    pub fn cursor_zoom(
        &mut self,
        ms: &sdl2::mouse::MouseState,
        scroll: i32,
        window: &SDL2Facade,
        camera_frame: Mat4) {
        // Zoom parameters
        const ZOOM_SPEED: f64 = 0.01;
        const LOWER_BOUND: f64 = 0.015;
        const UPPER_BOUND: f64 = 0.15;


        let mouse_scroll: f64 = scroll as f64;
        let zoom_scale: f64 = (mouse_scroll * ZOOM_SPEED).abs();
        let mouse_pos: &mut Vector2 = &mut Vector2 { x: ms.x() as f64, y: ms.y() as f64 };
        let camera_center: &mut Vector2 = &mut Vector2 { x: window.window().size().0 as f64 / 2.0, y: window.window().size().1 as f64 / 2.0 };

        translate_to_camera_coord(mouse_pos, window.window().size());
        translate_camera_to_world_coord(mouse_pos, camera_frame);

        translate_to_camera_coord(camera_center, window.window().size());
        translate_camera_to_world_coord(camera_center, camera_frame);


        // Zoom in to cursor
        if mouse_scroll > 0.0 && self.zoom.x < UPPER_BOUND {
            let old_zoom = self.zoom;
            let new_zoom = vector2(interpolate_zoom(zoom_scale, old_zoom.x, UPPER_BOUND), interpolate_zoom(zoom_scale, old_zoom.x, UPPER_BOUND));

            let mouse_vec = Vector2 { x: (mouse_pos.x - camera_center.x) * (new_zoom.x - old_zoom.x), y: (mouse_pos.y - camera_center.y) * (new_zoom.y - old_zoom.y) };



            if old_zoom.x != new_zoom.x || old_zoom.y != new_zoom.y {
                let delta_zoom = Vector2 { x: new_zoom.x / old_zoom.x, y: new_zoom.y / old_zoom.y };
                let camera_pos = Vector2 { x: self.position.x * delta_zoom.x, y: self.position.y * delta_zoom.y };
                let new_pos = camera_pos + mouse_vec;

                if is_mouse_within_bounds(*mouse_pos) {
                    self.position = new_pos;
                    self.zoom = new_zoom;
                }
            }

            // Zooming out from cursor
        } else if mouse_scroll < 0.0 && self.zoom.x > LOWER_BOUND {
            let old_zoom = self.zoom;
            let new_zoom = vector2(interpolate_zoom(zoom_scale, old_zoom.x, LOWER_BOUND), interpolate_zoom(zoom_scale, old_zoom.y, LOWER_BOUND));

            let mouse_vec = Vector2 { x: (mouse_pos.x - camera_center.x) * (new_zoom.x - old_zoom.x), y: (mouse_pos.y - camera_center.y) * (new_zoom.y - old_zoom.y) };

            if old_zoom.x != new_zoom.x || old_zoom.y != new_zoom.y {
                let delta_zoom = Vector2 { x: new_zoom.x / old_zoom.x, y: new_zoom.y / old_zoom.y };
                let camera_pos = Vector2 { x: self.position.x * delta_zoom.x, y: self.position.y * delta_zoom.y };
                let new_pos = camera_pos + mouse_vec;

                if is_mouse_within_bounds(*mouse_pos) {
                    self.position = new_pos;
                    self.zoom = new_zoom;
                }
            }
        }
    }
}

fn is_mouse_within_bounds(mouse_pos: Vector2) -> bool {
    // Bounding box
    const LEFT_BOUND: Scalar = -25.0;
    const RIGHT_BOUND: Scalar = 115.0;
    const TOP_BOUND: Scalar = 115.0;
    const BOTTOM_BOUND: Scalar = -25.0;

    let left_corner_bound = vector2(LEFT_BOUND, BOTTOM_BOUND);
    let right_corner_bound = vector2(RIGHT_BOUND, TOP_BOUND);

    if (mouse_pos.x > left_corner_bound.x && mouse_pos.y > left_corner_bound.y) && (mouse_pos.x < right_corner_bound.x && mouse_pos.y < right_corner_bound.y) {
        return true;
    } else {
        return false;
    }
}

// Smooth zooming
fn interpolate_zoom(value: f64, start: f64, end: f64) -> f64 {
    return start + (end - start) * value;
}

pub fn translate_to_camera_coord(vec: &mut Vector2, window_size: (u32, u32)) {
    vec.x = vec.x / window_size.0 as f64 * 2.0 - 1.0;
    vec.y = -(vec.y / window_size.1 as f64 * 2.0 - 1.0);
}

pub fn translate_camera_to_world_coord(vec: &mut Vector2, matrix: Mat4) {
    let inverse_matrix = matrix.inverse_matrix4();
    let temp_vec2 = Vector2 { x: vec.x, y: vec.y };
    let new_vec2 = inverse_matrix.multiply_vec2(temp_vec2);
    vec.x = new_vec2.x;
    vec.y = new_vec2.y;
}

pub fn translate_world_to_camera(vec: &mut Vector2, matrix: Mat4) {
    let temp_vec2 = Vector2 { x: vec.x, y: vec.y };
    let new_vec2 = matrix.multiply_vec2(temp_vec2);
    vec.x = new_vec2.x;
    vec.y = new_vec2.y;
}
