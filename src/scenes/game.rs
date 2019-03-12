use crate::simulation::state::State;
use crate::presentation::ui::gui::Component;
use crate::presentation::camera::Camera;
use crate::simulation::control::Control;

use crate::{presentation};

use glium_sdl2::SDL2Facade;
use sdl2::{EventPump};
use sdl2::keyboard::Keycode;

use crate::presentation::display::Programs;
use crate::presentation::display::Textures;
use glium::DrawParameters;
use crate::presentation::ui::glium_text::FontTexture;
use crate::simulation::game_state::GameState;
use crate::simulation;
use crate::scenes::scene::*;
use crate::scenes::{main_menu, loss_screen};

#[derive(Clone)]
pub struct Game {
    pub state: State,
    pub gui: Component,
    pub control: Control,
    pub camera: Camera,
    pub game_state: GameState,
}

impl Game {
    pub fn new() -> Game {
        let state = simulation::initial_state::initial_state(100, rand::random::<u32>());
        let gui = presentation::ui::gui::Component::init_game_gui();
        let camera = presentation::camera::Camera::new();
        let control = simulation::control::Control::new();
        let game_state = simulation::game_state::GameState::new();
        Game {
            state: state,
            gui: gui,
            control: control,
            camera: camera,
            game_state: game_state
        }
    }
}

impl Scene for Game {
    fn update(&mut self,
              event_pump: &mut EventPump,
              window: &SDL2Facade,
              delta_time: f64)
              -> UpdateResult {
        match self.game_state {
            GameState{terminate, transition_menu, transition_game, zombies_win, ..} =>
                {
                    if terminate {return UpdateResult::Exit}
                    if transition_game {self.game_state.transition_game = false;
                        return UpdateResult::Transition(Box::new(Game::new()))}
                    if transition_menu {self.game_state.transition_menu = false;
                        return UpdateResult::Transition(Box::new(main_menu::MainMenu::new()))}
                    if zombies_win {
                        self.game_state.tick += 1;
                        // wait 2 seconds
                        if self.game_state.tick == 120 {
                            self.game_state.zombies_win = false;
                            return UpdateResult::Transition(Box::new(loss_screen::LossScreen::new(self.clone().state)))
                        }
                    }
                }
        }
        let keyboard_state = event_pump.keyboard_state();
        let mouse_state = event_pump.mouse_state();
        self.camera.update(&keyboard_state, &mouse_state, &window, self.camera.compute_matrix(), delta_time);
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
                    println!("  Entity count:     {:?}", self.state.entities.len());
                    println!("  Projectile count: {:?}", self.state.projectiles.len());
                },
                Event::MouseWheel {timestamp: _, window_id: _, which: _, x: _, y, direction: _} => {
                    self.camera.set_zoom(&mouse_state, y, &window, self.camera.compute_matrix());
                },
                _ => {
                    self.gui.handle_event(event, &window, self.camera.compute_matrix(),
                                                            &mut self.state, &mut self.game_state,
                                                            &mut self.control);
                }
            }
        }

        if !self.game_state.game_paused {
            let sounds = simulation::update::update(
                &simulation::update::UpdateArgs { dt: delta_time },
                &mut self.state);
            presentation::audio::sound_effects::play_sounds(&sounds);
        }
        UpdateResult::Continue
    }

    fn render(&mut self,
              window:&SDL2Facade,
              programs:&Programs,
              textures:&Textures,
              params:&DrawParameters,
              font:&FontTexture) {

        let mut target = window.draw();
        presentation::display::display(&mut target,
                                       &window,
                                       &programs,
                                       &textures,
                                       &params,
                                       &self.state,
                                       self.camera.compute_matrix(),
                                       &mut self.gui, &font,
                                       &self.control);
        target.finish().unwrap();
    }
}
