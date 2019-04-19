use crate::simulation::update::EntityCounts;
use crate::simulation::game_state::GameState;
use crate::presentation::ui::gui::Component;
use crate::scenes::scene::{Scene, UpdateResult};
use sdl2::EventPump;
use glium_sdl2::SDL2Facade;
use crate::presentation::display::{Programs, Textures};
use glium::DrawParameters;
use crate::presentation::graphics::font::FontPkg;
use crate::{simulation, presentation};
use crate::scenes::{game, main_menu};
use sdl2::keyboard::Keycode;
use crate::presentation::camera::*;

pub struct DifficultyScreen {
    game_state: GameState,
    gui: Component,
    pub camera: Camera,
}

impl DifficultyScreen {
    pub fn new() -> DifficultyScreen {
        let game_state = simulation::game_state::GameState::new();
        let gui = presentation::ui::gui::Component::init_difficulty_gui();
        let camera = presentation::camera::Camera::new();
        DifficultyScreen {
            game_state: game_state,
            gui: gui,
            camera: camera,
        }
    }
}

impl Scene for DifficultyScreen {
    fn update(&mut self,
              event_pump: &mut EventPump,
              window: &mut SDL2Facade,
              delta_time: f64
    ) -> UpdateResult {
        match self.game_state {
            GameState{transition_game, transition_menu, terminate, ..} =>
                {
                    if transition_game {self.game_state.transition_game = false;
                        return UpdateResult::Transition(Box::new(game::Game::new(self.game_state.tutorial)))}
                    if transition_menu {self.game_state.transition_menu = false;
                        return UpdateResult::Transition(Box::new(main_menu::MainMenu::new()))}
                    if terminate {return UpdateResult::Exit}
                }
        }
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                // Exit window if escape key pressed or quit event triggered
                Event::Quit { .. } => {
                    return UpdateResult::Exit
                },
                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                    println!("Debug info:");
                    println!("  DT:               {:?}", delta_time);
                    println!("  FPS:              {:?}", 1.0 / delta_time);
                }
                _ => {
                    self.gui.handle_difficulty_event(&event, window, &mut self.game_state);
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
              fonts: &FontPkg
    ) {
        let mut target = window.draw();
        presentation::display::display_difficulty_screen(&mut target,
                                                         &window,
                                                         &programs,
                                                         &textures,
                                                         &params,
                                                         self.camera.compute_matrix().as_f32_array(),
                                                         &mut self.gui,

                                                         &fonts);
        target.finish().unwrap();
    }
}