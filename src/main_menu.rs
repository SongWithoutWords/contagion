use crate::presentation::ui::gui::Component;
use crate::scene::{Scene, UpdateResult};
use sdl2::EventPump;
use glium_sdl2::SDL2Facade;
use crate::presentation::display::{Programs, Textures};
use glium::DrawParameters;
use crate::presentation::ui::glium_text::FontTexture;
use crate::{presentation, simulation, game};
use sdl2::keyboard::Keycode;
use crate::simulation::game_state::GameState;

pub struct MainMenu {
    gui: Component,
    game_state: GameState,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        let gui = presentation::ui::gui::Component::init_main_menu_gui();
        let game_state = simulation::game_state::GameState::new();
        MainMenu {
            gui: gui,
            game_state: game_state
        }
    }
}

impl Scene for MainMenu {
    fn update(&mut self,
              event_pump: &mut EventPump,
              window: &SDL2Facade,
              delta_time: f64)
              -> UpdateResult {
        match self.game_state {
            GameState{transition_game, terminate, ..} =>
                {
                    if transition_game {self.game_state.transition_game = false;
                        return UpdateResult::Transition(Box::new(game::Game::new()))}
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
                    self.gui.handle_main_menu_event(&event, &window, &mut self.game_state);
                }
            }
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
        presentation::display::display_main_menu(&mut target,
                                       &window,
                                       &programs,
                                       &textures,
                                       &params,
                                       &mut self.gui, &font);
        target.finish().unwrap();
    }
}