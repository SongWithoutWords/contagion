use crate::simulation::state::State;
use crate::presentation::ui::gui::Component;
use crate::presentation::camera::Camera;
use crate::simulation::control::Control;

use crate::presentation;

use glium_sdl2::SDL2Facade;
use sdl2::{EventPump};
use sdl2::keyboard::Keycode;

use crate::presentation::display::Programs;
use crate::presentation::display::Textures;
use glium::DrawParameters;
use crate::presentation::ui::glium_text::FontTexture;
use crate::simulation::game_state::GameState;
use crate::simulation;
use crate::core::matrix::Mat4;
use crate::scene::Scene;


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SceneType {
    StartMenu,
    InGame,
    None,
}

pub static mut CURRENT_SCENE: SceneType = SceneType::InGame;
pub static mut PREV_SCENE: SceneType = SceneType::None;


/*In Game*/
pub struct Game {
    pub state: State,
    pub gui: Component,
    pub control: Control,
    pub camera: Camera,
    pub frame: Mat4,
    pub game_state: GameState,
}

impl Game {
    pub fn new() -> Game {
        let mut _state = simulation::initial_state::initial_state(100, rand::random::<u32>());
        let mut _gui = presentation::ui::gui::Component::init_game_gui();
        let mut _camera = presentation::camera::Camera::new();
        let mut _control = simulation::control::Control::new();
        let mut _game_state = simulation::game_state::GameState::new();
        let _frame = _camera.compute_matrix();
        Game {
            state: _state,
            gui: _gui,
            control: _control,
            camera: _camera,
            frame: _frame,
            game_state: _game_state
        }
    }
}

impl Scene for Game {
    fn handle_input(&mut self, event_pump: &mut EventPump, window:&SDL2Facade, delta_time: f64) {
        let keyboard_state = event_pump.keyboard_state();
        self.camera.update(&keyboard_state, delta_time);
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                // Exit window if escape key pressed or quit event triggered
                Event::Quit { .. } => {
                    break
                },
                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                    println!("Debug info:");
                    println!("  DT:               {:?}", delta_time);
                    println!("  FPS:              {:?}", 1.0 / delta_time);
                    println!("  Entity count:     {:?}", self.state.entities.len());
                    println!("  Projectile count: {:?}", self.state.projectiles.len());
                }
                Event::MouseWheel { timestamp: _, window_id: _, which: _, x: _, y, direction: _ } => {
                    self.camera.set_zoom(y);
                }
                _ => {
                    self.gui.handle_event(event, &window, self.frame,
                                                            &mut self.state, &mut self.game_state,
                                                            &mut self.control);
                }
            }
        }
    }
    fn update(&mut self, delta_time: f64) -> Option<Box<Scene>> {
        unsafe {
            if PREV_SCENE != CURRENT_SCENE {
                PREV_SCENE = SceneType::InGame;
                Some(Box::new(self::Game::new()))
            } else {
                // println!("updating ingame scene");
                self.frame = self.camera.compute_matrix();
                if !self.game_state.game_paused {
                    let _not_paused_game = simulation::update::update(
                        &simulation::update::UpdateArgs { dt: delta_time },
                        &mut self.state);
                }
                None
            }
        }
    }

    fn render(&mut self, window:&SDL2Facade, programs:&Programs, textures:&Textures, params:&DrawParameters, font:&FontTexture) {
        let mut target = window.draw();
        presentation::display::display(&mut target, &window, &programs, &textures, &params, &self.state,
                                       self.frame,  &mut self.gui, &font,
                                       &self.control);
        target.finish().unwrap();
    }
}

pub fn handle_scene_input(scene: &mut Scene, event_pump:&mut EventPump, window:&SDL2Facade, delta_time: f64) {
    scene.handle_input(event_pump, window, delta_time);
}

pub fn update_scene(scene: &mut Scene, delta_time: f64) -> Option<Box<Scene>>{
    scene.update(delta_time)
}

pub fn render_scene(scene: &mut Scene, window:&SDL2Facade, programs:&Programs, textures:&Textures, params:&DrawParameters, font:&FontTexture) {
    scene.render(window, programs, textures, params, font);
}