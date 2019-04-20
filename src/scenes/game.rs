use crate::simulation::state::State;
use crate::simulation::update::EntityCounts;
use crate::presentation::ui::gui::Component;
use crate::presentation::camera::Camera;
use crate::simulation::control::Control;

use crate::{presentation};

use glium_sdl2::SDL2Facade;
use sdl2::{EventPump};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use crate::presentation::display::Programs;
use crate::presentation::display::Textures;
use glium::DrawParameters;
use crate::simulation::game_state::GameState;
use crate::simulation;
use crate::scenes::scene::*;
use crate::scenes::main_menu;
use crate::scenes::difficulty_screen;
use crate::presentation::graphics::font::FontPkg;
use crate::scenes::victory_screen::VictoryScreen;
use crate::scenes::loss_screen::LossScreen;
use crate::core::scalar::Scalar;

pub struct Game {
    pub state: State,
    pub entity_counts: EntityCounts,
    pub gui: Component,
    pub control: Control,
    pub camera: Camera,
    pub game_state: GameState,
}

// Counts are out of 100%
const EASY_COP_COUNT: Scalar = 0.1;
const EASY_INFECTED_COUNT: Scalar = 0.07;
const MEDIUM_COP_COUNT: Scalar = 0.07;
const MEDIUM_INFECTED_COUNT: Scalar = 0.18;
const HARD_COP_COUNT: Scalar = 0.04;
const HARD_INFECTED_COUNT: Scalar = 0.4;

impl Game {
    pub fn new(tutorial: bool, difficulty: bool, easy: bool, medium: bool, hard: bool) -> Game {
        let gui = presentation::ui::gui::Component::init_game_gui();
        let camera = presentation::camera::Camera::new();
        let control = simulation::control::Control::new();
        let mut game_state: GameState;
        let entity_count = 100;
        let mut cop_entities = 0.05;
        let mut infected_entities = 0.2;

        // Set difficulty
        if easy {
            cop_entities = EASY_COP_COUNT;
            infected_entities = EASY_INFECTED_COUNT;
        } else if medium {
            cop_entities = MEDIUM_COP_COUNT;
            infected_entities = MEDIUM_INFECTED_COUNT;
        } else if hard {
            cop_entities = HARD_COP_COUNT;
            infected_entities = HARD_INFECTED_COUNT;
        }

        let state = simulation::initial_state::initial_state(entity_count, cop_entities, infected_entities, rand::random::<u32>());
        game_state = simulation::game_state::GameState::new();

        if difficulty {
            game_state = simulation::game_state::GameState::new_difficulty(easy, medium, hard);
        } else if tutorial {
            game_state = simulation::game_state::GameState::new_tutorial();
        }


        Game {
            state: state,
            entity_counts: EntityCounts::default(),
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
              window: &mut SDL2Facade,
              delta_time: f64)
              -> UpdateResult {
        match self.game_state {
            GameState{terminate, transition_menu, transition_game, zombies_win, humans_win, tutorial, summary_text, difficulty, easy, medium, hard, easy_game, medium_game, hard_game, ..} =>
                {
                    if terminate {
                        return UpdateResult::Exit
                    }
                    if transition_game {
                        self.game_state.transition_game = false;
                        return UpdateResult::Transition(Box::new(Game::new(self.game_state.tutorial, false, false, false, false)))
                    }
                    if transition_menu {self.game_state.transition_menu = false;
                        return UpdateResult::Transition(Box::new(main_menu::MainMenu::new()))
                    }
                    if easy_game {
                        self.game_state.easy_game = false;
                        self.game_state.easy = true;
                        return UpdateResult::Transition(Box::new(Game::new(false, true, self.game_state.easy, false, false)))
                    }
                    if medium_game {
                        self.game_state.medium_game = false;
                        self.game_state.medium = true;
                        return UpdateResult::Transition(Box::new(Game::new(false, true, false, self.game_state.medium, false)))
                    }
                    if hard_game {
                        self.game_state.hard_game = false;
                        self.game_state.hard = true;
                        return UpdateResult::Transition(Box::new(Game::new(false, true, false, false, self.game_state.hard)))
                    }
                    // Display difficulty selection screen
                    if difficulty {
                        self.game_state.difficulty = false;
                        return UpdateResult::Transition(Box::new(difficulty_screen::DifficultyScreen::new(easy, medium, hard)))
                    }
                    if summary_text {
                        self.game_state.fade_wait += 1;
                        // wait 5 seconds
                        if self.game_state.fade_wait == (60 * 5) {
                            self.game_state.summary_text = false;
                            self.game_state.fade_alpha = 1.0;
                        }
                    }
                    // display tutorial using game_state.rs flags
                    if tutorial {
                        // Let game display entities outside of buildings properly before
                        // displaying tutorial, then pause the game
                        self.game_state.tut_time_curr += 1;
                        if self.game_state.tut_time_curr == 2 {
                            self.game_state.game_paused = true;
                        }
                        // display tutorial 1: display text to teach about selecting
                        if self.game_state.tut_01 == true {
                            // stub
                        }
                        // display tutorial 2: display text to teach about targetting and moving
                        if self.game_state.tut_02 == true {
                            // stub
                        }
                        // display tutorial 3: display text to teach about unpausing and pausing
                        if self.game_state.tut_03 == true {
                            // stub
                        }
                    }
                    if zombies_win {
                        self.game_state.trans_wait += 1;
                        // wait 2 seconds
                        if self.game_state.trans_wait == 120 {
                            self.game_state.zombies_win = false;
                            return UpdateResult::Transition(Box::new(LossScreen::new(self.entity_counts.clone(), easy, medium, hard)))
                        }
                    }
                    if humans_win {
                        self.game_state.trans_wait += 1;
                        // wait 2 seconds
                        if self.game_state.trans_wait == 120 {
                            self.game_state.humans_win = false;
                            return UpdateResult::Transition(Box::new(VictoryScreen::new(self.entity_counts.clone(), easy, medium, hard)))
                        }
                    }
                }
        }
        let keyboard_state = event_pump.keyboard_state();
        let mouse_state = event_pump.mouse_state();
        self.camera.update(&keyboard_state, &mouse_state, window, self.camera.compute_matrix(), delta_time);
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
                    self.camera.cursor_zoom(&mouse_state, y, &window, self.camera.compute_matrix());
                },
                Event::MouseButtonDown {timestamp: _, window_id: _, which: _, mouse_btn: MouseButton::Middle , x, y} => {
                    self.camera.set_initial_mouse_pos(x, y, &window, self.camera.compute_matrix());
                },
                _ => {
                    self.gui.handle_event(event, window, self.camera.compute_matrix(),
                                                            &mut self.state, &mut self.game_state,
                                                            &mut self.control);
                }
            }
        }

        if !self.game_state.game_paused {
            let simulation_results = simulation::update::update(
                &simulation::update::UpdateArgs { dt: delta_time },
                &mut self.state);
            self.entity_counts = simulation_results.entity_counts;

            if self.entity_counts.infected == 0 && self.entity_counts.zombies == 0 {
                // The player wins if there are no zombies or infected
                self.game_state.humans_win = true;
            }
            else if self.entity_counts.cops == 0 {
                // The player loses if there are still zombies or infected,
                // and no cops to defend the remaining civilians
                self.game_state.zombies_win = true;
            }

            presentation::audio::sound_effects::play_sounds(&simulation_results.sounds, &self.camera);
        }
        UpdateResult::Continue
    }

    fn render(&mut self,
              window:&SDL2Facade,
              programs:&Programs,
              textures:&Textures,
              params:&DrawParameters,
              fonts:&FontPkg) {

        let mut target = window.draw();
        presentation::display::display(&mut target,
                                       &window,
                                       &programs,
                                       &textures,
                                       &params,
                                       &self.state,
                                       &self.entity_counts,
                                       self.camera.compute_matrix(),
                                       &mut self.gui, &fonts,
                                       &self.control,
                                       &mut self.game_state);
        target.finish().unwrap();
    }
}
