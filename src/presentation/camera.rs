use crate::core::scalar::*;
use crate::core::vector::*;
use crate::core::matrix::*;
use crate::simulation::control::*;
use glium_sdl2::SDL2Facade;

pub struct Camera {
    position: Vector2,
    velocity: Vector2,
    zoom: Vector2,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: Vector2::zero(),
            velocity: Vector2::zero(),
            zoom: Vector2 {x: 0.09 as f64, y: 0.09 as f64}  // set initial zoom level
        }
    }

    const DRAG_FACTOR: Scalar = 8.0;
    const ACCELERATION_FACTOR: Scalar = 10.0;

    pub fn update(
        &mut self,
        ks: &sdl2::keyboard::KeyboardState,
        delta_time: Scalar) {

        use sdl2::keyboard::Scancode;
        fn key_pressed(ks: &sdl2::keyboard::KeyboardState, s: Scancode) -> Scalar {
            if ks.is_scancode_pressed(s) {
                1.0
            } else {
                0.0
            }
        }

        let acceleration = Vector2 {
            x: key_pressed(ks, Scancode::D) - key_pressed(ks, Scancode::A),
            y: key_pressed(ks, Scancode::W) - key_pressed(ks, Scancode::S),
        };

        self.velocity += delta_time * Self::ACCELERATION_FACTOR * acceleration;
        self.position += delta_time * self.velocity;
        self.velocity -= delta_time * Self::DRAG_FACTOR * self.velocity;


    }

    // Camera frame in world coordinates
    pub fn compute_matrix(&self) -> Mat4 {
        (Mat4 {
            i : Vector4 {x: self.zoom.x, y: 0.0, z: 0.0, w: 0.0},
            j : Vector4 {x: 0.0, y: self.zoom.y, z: 0.0, w: 0.0},
            k : Vector4 {x: 0.0, y: 0.0, z: 1.0, w: 0.0},
            w : Vector4 {x: -self.position.x, y: -self.position.y, z: 0.0, w: 1.0},
        })
    }

    pub fn set_zoom(
        &mut self,
        ms: &sdl2::mouse::MouseState,
        scroll: i32,
        window: &SDL2Facade,
        camera_frame: Mat4) {

        const ZOOM_SPEED: f64 = 0.0005;
        const LOWER_BOUND: f64 = 0.015;
        const UPPER_BOUND: f64 = 0.15;

        let mouse_scroll: f64 = scroll as f64;
        let zoom_scale: f64 = (mouse_scroll * ZOOM_SPEED).abs();
        let mouse_pos: &mut Vector2 = &mut Vector2 { x: ms.x() as f64, y: ms.y() as f64 };
        let camera_center: &mut Vector2 = &mut Vector2 {x: window.window().size().0 as f64 / 2.0, y: window.window().size().1 as f64 / 2.0};

        translate_to_camera(mouse_pos, window.window().size());
        translate_camera_to_world(mouse_pos, camera_frame);

        translate_to_camera(camera_center, window.window().size());
        translate_camera_to_world(camera_center, camera_frame);



        // Zoom in to cursor
        if mouse_scroll > 0.0 && self.zoom.x < UPPER_BOUND - 0.05 {

            let old_zoom = self.zoom;
            let new_zoom = Vector2 {x: old_zoom.x + zoom_scale, y: old_zoom.y + zoom_scale};

            let delta_cord = Vector2 {x: (mouse_pos.x - camera_center.x) * (new_zoom.x - old_zoom.x), y: (mouse_pos.y - camera_center.y) * (new_zoom.y - old_zoom.y)};

            println!("{}", "delta cord");
            println!("{:?}", delta_cord);

            if old_zoom.x != new_zoom.x || old_zoom.y != new_zoom.y {
                let scale_delta_cord = Vector2 {x: new_zoom.x / old_zoom.x, y: new_zoom.y / old_zoom.y};
                let camera_pos = Vector2 {x: self.position.x * scale_delta_cord.x, y: self.position.y * scale_delta_cord.y};
                self.position = camera_pos + delta_cord;
                self.zoom = new_zoom;
            }








////            let new_zoom = vector2(interpolate_zoom(zoom_scale, self.zoom.x, UPPER_BOUND), interpolate_zoom(zoom_scale, self.zoom.x, UPPER_BOUND));
//
//            let new_zoom = vector2(self.zoom.x + zoom_scale, self.zoom.y + zoom_scale);
//
//
////            mouse_pos.x *= new_zoom.x;
////            mouse_pos.y *= new_zoom.y;
//
////            camera_center.x *= new_zoom.x;
////            camera_center.y *= new_zoom.y;
//
//            let move_vec = &mut Vector2 {x: mouse_pos.x - camera_center.x, y: mouse_pos.y - camera_center.y};
//
//
//            self.zoom = new_zoom;
//            self.position = self.position + *move_vec;

            // Zooming out from camera_center of camera
        } else if mouse_scroll < 0.0 && self.zoom.x > LOWER_BOUND {

//            let new_zoom = vector2(interpolate_zoom(zoom_scale, self.zoom.x, LOWER_BOUND), interpolate_zoom(zoom_scale, self.zoom.x, LOWER_BOUND));

            let new_zoom = vector2(self.zoom.x - zoom_scale, self.zoom.y - zoom_scale);
            camera_center.x *= new_zoom.x;
            camera_center.y *= new_zoom.y;

            self.zoom = new_zoom;
            self.position = *camera_center;
        }


//        if mouse_scroll > 0.0 && self.zoom < UPPER_BOUND {
//            let new_zoom = self.interpolate_zoom(zoom_scale, self.zoom, LOWER_BOUND);
//
//            // TODO: Change 0.5 to mouse position in screen coord

//
//            // Offset the camera to keep it centered
//            let delta_zoom_inverse = self.zoom / new_zoom;
//
//            let height = camera_frame.j.y;
//            let width = camera_frame.i.x;
//
//            let x_offset_factor = (1.0 - delta_zoom_inverse) * 0.5;
//            let y_offset_factor = (1.0 - delta_zoom_inverse) * 0.5;
//
//            self.position += vector2(x_offset_factor, y_offset_factor);
//            self.zoom = new_zoom;
    }
}

// Smooth zooming
fn interpolate_zoom(value: f64, start: f64, end: f64) -> f64 {
    return start + (end - start) * value;
}

pub fn translate_to_camera(vec: &mut Vector2, window_size: (u32, u32)) {
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
