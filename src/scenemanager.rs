use crate::simulation::state::State;
use crate::presentation::ui::gui::Component;
use crate::presentation::camera::Camera;
use crate::simulation::control::Control;

use crate::simulation::initial_state::initial_state;
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


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SceneType {
    StartMenu,
    InGame,
    None,
}

pub static mut CURRENT_SCENE: SceneType = SceneType::InGame;
pub static mut PREV_SCENE: SceneType = SceneType::None;

pub trait Scene {
    fn handle_input(&mut self, event_pump:&mut EventPump, window:&SDL2Facade, delta_time: f64);
    fn update(&mut self, event_pump: &EventPump, delta_time: f64) -> Option<Box<Scene>>;
    fn render(&mut self, window:&SDL2Facade, program:&Programs, textures:&Textures, params:&DrawParameters, font:&FontTexture);
}

///*Start Menu*/
//pub struct StartMenu {
//    gui: Component,
//}
//
//impl Scene for StartMenu {
//    fn handle_input(&self){}
//    fn update(&self) -> Box<Scene> {
//        None
//    }
//    fn render(&self){}
//}

/*In Game*/
pub struct InGame {
    pub state: Option<State>,
    pub gui: Option<Component>,
    pub control: Option<Control>,
    pub camera: Option<Camera>,
    pub frame: Option<Mat4>,
    pub game_state: Option<GameState>,
}

impl InGame {
    pub fn new() -> InGame {
        InGame {
            state: None,
            gui: None,
            control: None,
            camera: None,
            frame: None,
            game_state: None,
        }
    }
}

impl Scene for InGame {
    fn handle_input(&mut self, event_pump: &mut EventPump, window:&SDL2Facade, delta_time: f64) {
        println!("handling input");
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
                    println!("  Entity count:     {:?}", self.state.as_ref().unwrap().entities.len());
                    println!("  Projectile count: {:?}", self.state.as_ref().unwrap().projectiles.len());
                }
                Event::MouseWheel { timestamp: _, window_id: _, which: _, x: _, y, direction: _ } => {
                    self.camera.as_mut().unwrap().set_zoom(y);
                }
                _ => {
                    self.gui.as_mut().unwrap().handle_event(event, &window, self.frame.unwrap(),
                                                            self.state.as_mut().unwrap(), self.game_state.as_mut().unwrap(),
                                                            self.control.as_mut().unwrap());
                }
            }
        }
    }
    fn update(&mut self, ref mut event_pump: &EventPump, delta_time: f64) -> Option<Box<Scene>> {
        unsafe {
            if PREV_SCENE != CURRENT_SCENE {
                PREV_SCENE = SceneType::InGame;
                let mut state = initial_state(100, rand::random::<u32>());
                let gui = Component::init_demo();
                let control = Control::new();
                let mut camera = Camera::new();
                let keyboard_state = event_pump.keyboard_state();
                let mut game_state = simulation::game_state::GameState::new();
                camera.update(&keyboard_state, delta_time);
                let frame = camera.compute_matrix();
                if !game_state.game_paused {
                    let _not_paused_game = simulation::update::update(
                        &simulation::update::UpdateArgs { dt: delta_time },
                        &mut state);
                }
                println!("initializing ingame scene");
                let returntype = Box::new(self::InGame {
                    state: Some(state),
                    gui: Some(gui),
                    control: Some(control),
                    camera: Some(camera),
                    frame: Some(frame),
                    game_state: Some(game_state)
                });
                Some(returntype)
            }
            else {
                println!("updating ingame scene");
                let keyboard_state = event_pump.keyboard_state();
                self.camera.as_mut().unwrap().update(&keyboard_state, delta_time);
                if !self.game_state.as_ref().unwrap().game_paused {
                    let _not_paused_game = simulation::update::update(
                        &simulation::update::UpdateArgs { dt: delta_time },
                        self.state.as_mut().unwrap());
                }
                None
            }
        }
    }
    fn render(&mut self, window:&SDL2Facade, programs:&Programs, textures:&Textures, params:&DrawParameters, font:&FontTexture) {
        println!("rendering ingame scene");
        let mut target = window.draw();
        presentation::display::display(&mut target, &window, &programs, &textures, &params, &self.state.as_ref().unwrap(),
                                       self.frame.unwrap(),  self.gui.as_mut().unwrap(), &font,
                                       &self.control.as_ref().unwrap());
        target.finish().unwrap();
    }
}

pub fn handle_scene_input(scene: &mut Scene, event_pump:&mut EventPump, window:&SDL2Facade, delta_time: f64) {
    scene.handle_input(event_pump, window, delta_time);
}

pub fn update_scene(scene: &mut Scene, event_pump: &EventPump, delta_time: f64) -> Option<Box<Scene>>{
    scene.update(event_pump, delta_time)
}

pub fn render_scene(scene: &mut Scene, window:&SDL2Facade, programs:&Programs, textures:&Textures, params:&DrawParameters, font:&FontTexture) {
    scene.render(window, programs, textures, params, font);
}