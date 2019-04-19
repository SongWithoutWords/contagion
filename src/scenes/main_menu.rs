use glium::DrawParameters;
use glium_sdl2::SDL2Facade;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;

use crate::{presentation, simulation};
use crate::presentation::display::{Programs, Textures};
use crate::presentation::graphics::font::FontPkg;
use crate::presentation::ui::gui::Component;
use crate::scenes::game;
use crate::scenes::difficulty_screen;
use crate::scenes::scene::{Scene, UpdateResult};
use crate::simulation::game_state::GameState;
use crate::presentation::camera::*;

pub struct MainMenu {
    gui: Component,
    game_state: GameState,
    pub camera: Camera,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        let gui = presentation::ui::gui::Component::init_main_menu_gui();
        let game_state = simulation::game_state::GameState::new();
        let camera = presentation::camera::Camera::new();
        MainMenu {
            gui: gui,
            game_state: game_state,
            camera: camera,
        }
    }
}

impl Scene for MainMenu {
    fn update(&mut self,
              event_pump: &mut EventPump,
              window: &mut SDL2Facade,
              delta_time: f64)
              -> UpdateResult {
        match self.game_state {
            GameState { transition_game, difficulty, easy, medium, hard, terminate, .. } =>
                {

                    if difficulty {
                        self.game_state.difficulty = false;
                        return UpdateResult::Transition(Box::new(difficulty_screen::DifficultyScreen::new(easy, medium, hard)))
                    }
                    if transition_game {
                        self.game_state.transition_game = false;
                        return UpdateResult::Transition(Box::new(game::Game::new(self.game_state.tutorial, self.game_state.difficulty, self.game_state.easy_game, self.game_state.medium_game, self.game_state.hard_game)));
                    }
                    if terminate { return UpdateResult::Exit; }
                }
        }
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                // Exit window if escape key pressed or quit event triggered
                Event::Quit { .. } => {
                    return UpdateResult::Exit;
                }
                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                    println!("Debug info:");
                    println!("  DT:               {:?}", delta_time);
                    println!("  FPS:              {:?}", 1.0 / delta_time);
                }
                _ => {
                    self.gui.handle_main_menu_event(&event, window, &mut self.game_state);
                }
            }
        }
        UpdateResult::Continue
    }

    fn render(&mut self,
              window: &SDL2Facade,
              programs: &Programs,
              textures: &Textures,
              params: &DrawParameters,
              fonts: &FontPkg) {
        let mut target = window.draw();
        presentation::display::display_main_menu(&mut target,
                                                 &window,
                                                 &programs,
                                                 &textures,
                                                 &params,
                                                 self.camera.compute_matrix().as_f32_array(),
                                                 &mut self.gui, &fonts);
        target.finish().unwrap();
    }
}