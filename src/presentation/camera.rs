use crate::core::scalar::*;
use crate::core::vector::*;
use crate::core::matrix::*;
use crate::simulation::control::*;

use crate::simulation::state::State;

use glium_sdl2::SDL2Facade;
use sdl2::event::Event;
use sdl2::mouse::MouseState;
use sdl2::mouse::Cursor;

pub struct Camera {
    position: Vector2,
    velocity: Vector2,
    zoom: Scalar,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: Vector2::zero(),
            velocity: Vector2::zero(),
            zoom: 0.09,
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

        Event::MouseWheel {}

        let acceleration = Vector2 {
            x: key_pressed(ks, Scancode::D) - key_pressed(ks, Scancode::A),
            y: key_pressed(ks, Scancode::W) - key_pressed(ks, Scancode::S),
        };

        self.velocity += delta_time * Self::ACCELERATION_FACTOR * acceleration;
        self.position += delta_time * self.velocity;
        self.velocity -= delta_time * Self::DRAG_FACTOR * self.velocity;


    }

    pub fn set_zoom(
        &mut self,
        ms: &sdl2::mouse::MouseState,
        scroll: i32,
        window: &SDL2Facade,
        camera_frame: Mat4) {

        const ZOOM_SPEED: Scalar = 0.0005;
        const LOWER_BOUND: Scalar = 0.015;
        const UPPER_BOUND: Scalar = 0.15;

        let mouse_scroll = scroll as Scalar;
        let zoom_scale = mouse_scroll * ZOOM_SPEED;
        let zoom = zoom_scale.abs();
        let mouse_pos = &mut Vector2 { x: ms.x() as f64, y: ms.y() as f64 };


        translate_mouse_to_camera(mouse_pos, window.window().size());
        println!("{}", "Mouse Position");
        println!("{:?}", mouse_pos);


        //translate_camera_to_world(mouse_pos, camera_frame);

//        println!("{:?}", zoom);

//        println!("{}", "Camera Position");
//        println!("{:?}", self.position);

//        println!("{}", "Mouse Position");
//        println!("{:?}", mouse_pos);

//        println!("{}", "MouseZoom Position");
//        println!("{:?}", mouse_pos.x * 0.1);
//        println!("{:?}", mouse_pos.y * 0.1);

        //TODO: Mouse coord to world coord

        // Limit camera zoom
        if zoom_scale > 0.0 && self.zoom < UPPER_BOUND {
            //camera_frame.i.x += zoom;
            //camera_frame.w.x = *mouse_pos.x;
//            self.position.x += mouse_pos.x * 0.01; // * zoom_scale
//            self.position.y += mouse_pos.y * 0.01;
            self.zoom += zoom;
        } else if mouse_scroll < 0.0 && self.zoom > LOWER_BOUND {
            //self.position.x += mouse_pos.x * 0.01;
            //self.position.y += mouse_pos.y * 0.01;
            self.zoom -= zoom;
        }
    }

    pub fn compute_matrix(&self) -> Mat4 {
        (Mat4 {
            i : Vector4 {x: self.zoom, y: 0.0, z: 0.0, w: 0.0},
            j : Vector4 {x: 0.0, y: self.zoom, z: 0.0, w: 0.0},
            k : Vector4 {x: 0.0, y: 0.0, z: 1.0, w: 0.0},
            w : Vector4 {x: -self.position.x as f64, y: -self.position.y as f64, z: 0.0, w: 1.0},
        })
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
}
