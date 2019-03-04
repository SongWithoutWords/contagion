use crate::core::scalar::*;
use crate::core::vector::*;
use crate::core::matrix::*;

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
        let acceleration = Vector2 {
            x: key_pressed(ks, Scancode::D) - key_pressed(ks, Scancode::A),
            y: key_pressed(ks, Scancode::W) - key_pressed(ks, Scancode::S),
        };

        self.velocity += delta_time * Self::ACCELERATION_FACTOR * acceleration;
        self.position += delta_time * self.velocity;
        self.velocity -= delta_time * Self::DRAG_FACTOR * self.velocity;
    }

    pub fn compute_matrix(&self) -> Mat4 {
        (Mat4 {
            i : Vector4 {x: self.zoom, y: 0.0, z: 0.0, w: 0.0},
            j : Vector4 {x: 0.0, y: self.zoom, z: 0.0, w: 0.0},
            k : Vector4 {x: 0.0, y: 0.0, z: 1.0, w: 0.0},
            w : Vector4 {x: -self.position.x as f64, y: -self.position.y as f64, z: 0.0, w: 1.0},
        })
    }

    // Set camera zoom level
    pub fn set_zoom(&mut self, y: i32) {
        const SCALE_FACTOR: Scalar = 0.0005;
        const LOWER_BOUND: Scalar = 0.015;
        const UPPER_BOUND: Scalar = 0.15;

        let mouse_scroll = y as Scalar;
        let zoom_scale = mouse_scroll * SCALE_FACTOR;
        let zoom = zoom_scale.abs();

        // Limit camera zoom
        if zoom_scale > 0.0 && self.zoom < UPPER_BOUND {
            self.zoom += zoom;
        } else if mouse_scroll < 0.0 && self.zoom > LOWER_BOUND {
            self.zoom -= zoom;
        }
    }
}
