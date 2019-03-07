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

        const ZOOM_SPEED: f64 = 0.005;
        const LOWER_BOUND: f64 = 0.015;
        const UPPER_BOUND: f64 = 0.15;

        let mouse_scroll: f64 = scroll as f64;
        let zoom_scale: f64 = (mouse_scroll * ZOOM_SPEED).abs();
        let mouse_pos: &mut Vector2 = &mut Vector2 { x: ms.x() as f64, y: ms.y() as f64 };

        let center: &mut Vector2 = &mut Vector2 {x: window.window().size().0 as f64 / 2.0, y: window.window().size().1 as f64 / 2.0};
        println!("{}", "center window coord");
        println!("{:?}", center);

        translate_mouse_to_camera(center, window.window().size());

        println!("{}", "center camera coord");
        println!("{:?}", center);

        println!("{}", "camera frame");
        println!("{:?}", camera_frame);

        translate_camera_to_world(center, camera_frame);



        if mouse_scroll > 0.0 && self.zoom.x < UPPER_BOUND {
            println!("{}", "center world coord");
            println!("{:?}", center);

            let world_pos_scale = Vector2 {x: self.zoom.x + zoom_scale, y: self.zoom.y + zoom_scale};

            println!("{}", "center scale");
            println!("{:?}", world_pos_scale);

            center.x *= world_pos_scale.x;
            center.y *= world_pos_scale.y;

            println!("{}", "center world scaled coord");
            println!("{:?}", center);

            self.zoom = Vector2 {x: self.zoom.x + zoom_scale, y: self.zoom.y + zoom_scale};
            println!("{}", "self.zoom");
            println!("{:?}", self.zoom);

            self.position = *center;
        } else if mouse_scroll < 0.0 && self.zoom.x > LOWER_BOUND {
            println!("{}", "center world coord");
            println!("{:?}", center);

            let world_pos_scale = Vector2 {x: self.zoom.x - zoom_scale, y: self.zoom.y - zoom_scale};

            println!("{}", "center scale");
            println!("{:?}", world_pos_scale);

            center.x *= world_pos_scale.x;
            center.y *= world_pos_scale.y;

            println!("{}", "center world scaled coord");
            println!("{:?}", center);

            self.zoom = Vector2 {x: self.zoom.x - zoom_scale, y: self.zoom.y - zoom_scale};
            println!("{}", "self.zoom");
            println!("{:?}", self.zoom);

            self.position = *center;
        }



        //translate_mouse_to_camera(mouse_pos, window.window().size());
        //translate_camera_to_world(mouse_pos, camera_frame);

        //let camera_world_pos = Vector2 {x: -camera_frame.w.x, y: -camera_frame.w.y};
        //let mouse_world_pos = &mut Vector2 {x: mouse_pos.x, y: mouse_pos.y};

        //let world_pos_scale = Vector2 {x: camera_world_pos.x / mouse_world_pos.x, y: camera_world_pos.y / mouse_world_pos.y};

//        println!("{}", "Camera World Position");
//        println!("{:?}", camera_world_pos);

        //let world_pos_scale = Vector2 {x: 0.898678771250003 / 10.031551765622623, y: 0.9381503312395915 / 10.423892569328794};
        //mouse_world_pos.x *= world_pos_scale.x;
        //mouse_world_pos.y *= world_pos_scale.y;

//        println!("{}", "Mouse World Position");
//        println!("{:?}", mouse_world_pos);

//        let old_zoom = self.zoom;
//        let new_zoom = old_zoom + zoom_scale;
//        self.zoom = new_zoom;

//        if mouse_scroll > 0.0 && self.zoom < UPPER_BOUND {
//            let new_zoom = self.interpolate_zoom(zoom_scale, self.zoom, LOWER_BOUND);
//
//            // TODO: Find camera frame width and height
//            // TODO: Change 0.5 to mouse position in screen coord

        // TODO: Get window coord convert to camera coord convert to world coord, set as position on each frame
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
//
//
//        } else if mouse_scroll < 0.0 && self.zoom > LOWER_BOUND {
//            let new_zoom = self.interpolate_zoom(zoom_scale, self.zoom, UPPER_BOUND);
//
//            // TODO: Find camera frame width and height
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
//        }

    }

    // Smooth zooming
    fn interpolate_zoom(&mut self, value: f64, start: f64, end: f64) -> f64 {
        return start + (end - start) * value;
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
