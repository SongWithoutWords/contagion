use crate::presentation::{ui::gui, camera};
use crate::simulation::state::State;
use crate::presentation::ui::gui::Component;
use crate::presentation::camera::Camera;
use crate::simulation::control::Control;

use crate::simulation::initial_state::initial_state;
use crate::core::matrix::Mat4;
use crate::presentation;
use std::fs::File;
use std::time::Instant;

use glium::draw_parameters::Blend;
use glium_sdl2::SDL2Facade;
use sdl2::{EventPump, Sdl};
use sdl2::keyboard::Keycode;

use crate::core::scalar:: *;
use crate::core::vector:: *;
use crate::presentation::audio::sound_effects:: *;
use crate::presentation::ui::glium_text;
use crate::presentation::ui::gui::{CURRENT,ActiveWindow};
use sdl2::mixer::query_spec;
use crate::presentation::display::Programs;
use crate::presentation::display::Textures;
use glium::DrawParameters;
use crate::presentation::ui::glium_text::FontTexture;


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SceneType {
    StartMenu,
    InGame,
    None,
}

pub static mut CURRENT_SCENE: SceneType = SceneType::InGame;
pub static mut PREV_SCENE: SceneType = SceneType::None;

pub trait Scene {
    fn handle_input(&self);
    fn update(&self) -> Option<Box<Scene>>;
    fn render(&self, window:SDL2Facade, program:Programs, textures:Textures, params:DrawParameters, font:FontTexture);
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
}

impl InGame {
    pub fn new() -> InGame {
        InGame {
            state: None,
            gui: None,
            control: None,
            camera: None,
        }
    }
}

impl Scene for InGame {
    fn handle_input(&self, event_pump, terminate, ) {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                // TODO: refactor into control
                // Exit window if escape key pressed or quit event triggered
                Event::Quit { .. } => {
                    break 'main_game_loop;
                }
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    game_paused = !game_paused;
                },
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    unsafe {
                        if CURRENT == ActiveWindow::Menu {
                            CURRENT = ActiveWindow::Game;
                            game_paused = false;
                        }
                        else if CURRENT == ActiveWindow::Instruction {
                            CURRENT = ActiveWindow::Menu;
                        }
                        else if CURRENT == ActiveWindow::Game {
                            CURRENT = ActiveWindow::Menu;
                            game_paused = true;
                        }
                    }

                },
//                    Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => {
//                        unsafe {
//                            if CURRENT == ActiveWindow::Menu {
//                                CURRENT = ActiveWindow::Game;
//                                game_paused = !game_paused;
//                            }
//                            else if CURRENT == ActiveWindow::Instruction {
//                                CURRENT = ActiveWindow::Menu;
//                            }
//                            else if CURRENT == ActiveWindow::Game {
//                                CURRENT = ActiveWindow::Menu;
//                                game_paused = !game_paused;
//                            }
//                        }
//                    },
                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                    println!("Debug info:");
                    println!("  DT:               {:?}", delta_time);
                    println!("  FPS:              {:?}", 1.0 / delta_time);
                    println!("  Entity count:     {:?}", state.entities.len());
                    println!("  Projectile count: {:?}", state.projectiles.len());
                }
                Event::MouseWheel { timestamp: _, window_id: _, which: _, x: _, y, direction: _ } => {
                    camera.set_zoom(y);
                }
                _ => {
                    ui.handle_event(event, &mut control, &window, camera_frame, &mut state, &mut game_paused, &mut terminate);
                }
            }
        }
    }
    fn update(&self) -> Option<Box<Scene>> {
        unsafe {
            if PREV_SCENE != CURRENT_SCENE {
                PREV_SCENE = SceneType::InGame;
                Some(Box::new(self::InGame {
                    state: Some(initial_state(100, rand::random::<u32>())),
                    gui: Some(Component::init_demo()),
                    control: Some(Control::new()),
                    camera: Some(Camera::new()),
                }))
            }
            else {
                None
            }
        }
    }
    fn render(&self, window:SDL2Facade, programs:Programs, textures:Textures, params:DrawParameters, font:FontTexture) {
        let mut target = window.draw();
        presentation::display::display(&mut target, &window, &programs, &textures, &params, &self.state.as_ref().unwrap(),
                                       self.camera.as_ref().unwrap().compute_matrix(), &self.gui.as_ref().unwrap(), &font,
                                       &self.control.as_ref().unwrap());
        target.finish().unwrap();
    }
}

pub fn update_scene(x: &Scene) -> Option<Box<Scene>>{
    x.update()
}