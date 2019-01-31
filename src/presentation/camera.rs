use crate::core::vector::*;

pub struct Camera {
    position: (f32, f32),
    direction: (f32, f32),
    pub w_down: bool,
    pub s_down: bool,
    pub a_down: bool,
    pub d_down: bool,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: (0.0, 0.0),
            direction: (0.0, 0.0),
            w_down: false,
            s_down: false,
            a_down: false,
            d_down: false,
        }
    }

    fn set_direction(&mut self, x: f32, y: f32) {
        // x and y inversed in camera coord
        let x = -x;
        let y = -y;
        self.direction = (x, y);
    }

    fn set_position(&mut self, x: f32, y: f32) {
        // x and y inversed in camera coord
        let x = -x;
        let y = -y;
        self.position = (x, y);
    }

    pub fn get_direction(&mut self) -> (f32, f32) {
        (self.direction)
    }

    pub fn get_position(&mut self) -> (f32, f32) {
        (self.position)
    }

    pub fn update(&mut self, elapsed_time: f32) -> [[f32;4];4] {
        // W down
        if self.w_down {
            self.set_direction(0.0, 1.0);
//            // W & D
//            if self.d_down {
//                self.set_direction(1.0, 1.0);
//            };
//            // W & A
//            if self.a_down {
//                self.set_direction(-1.0, 1.0);
//            }
        }
        // S down
        if self.s_down {
            self.set_direction(0.0, -1.0);
            // S & A
//            if self.a_down {
//                self.set_direction(-1.0, -1.0);
//            };
//            // S & D
//            if self.d_down {
//                self.set_direction(1.0, -1.0);
//            }
        }
        // A down
        if self.a_down {
            self.set_direction(-1.0, 0.0);
            // A & W
//            if self.w_down {
//                self.set_direction(-1.0, 1.0);
//            };
//            // A & S
//            if self.s_down {
//                self.set_direction(-1.1, -1.1);
//            }
        }
        // D down
        if self.d_down {
            self.set_direction(1.0, 0.0);
            // D & S
//            if self.d_down {
//                self.set_direction(1.0, -1.0);
//            };
//            // D & W
//            if self.a_down {
//                self.set_direction(1.0, 1.0);
//            }
        }

        let dragFactor = 0.1;
        let mut x_dir = self.get_direction().0;
        let mut y_dir  = self.get_direction().1;
        let mut x_pos = self.get_position().0;
        let mut y_pos = self.get_position().1;
        x_pos += x_dir * dragFactor * elapsed_time;
        y_pos += y_dir * dragFactor * elapsed_time;
        self.set_position(x_pos, y_pos);
        let mut camera_frame = [
            [0.1, 0.0, 0.0, 0.0],
            [0.0, 0.1, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [x_pos, y_pos, 0.0, 1.0f32],
        ];
        (camera_frame)
    }

}