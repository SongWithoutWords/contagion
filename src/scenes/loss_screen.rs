use crate::simulation::state::{State, Entity};
use crate::simulation::game_state::GameState;
use crate::presentation::ui::gui::Component;
use crate::scenes::scene::{Scene, UpdateResult};
use sdl2::EventPump;
use glium_sdl2::SDL2Facade;
use crate::presentation::display::{Programs, Textures};
use glium::DrawParameters;
use crate::presentation::ui::glium_text::FontTexture;
use crate::{simulation, presentation};
use glium::buffer::Content;

pub struct LossScreen {
    state: State,
    game_state: GameState,
    gui: Component,
}

impl LossScreen {
    pub fn new(state: State) -> LossScreen {
        let game_state = simulation::game_state::GameState::new();
        let gui = presentation::ui::gui::Component::init_loss_gui();
        LossScreen {
            state: state,
            game_state: game_state,
            gui:gui,
        }
    }
}

impl Scene for LossScreen  {
    fn update(&mut self, _event_pump: &mut EventPump, _window: &SDL2Facade, _delta_time: f64) -> UpdateResult {
        unimplemented!()
    }

    fn render(&mut self, _window: &SDL2Facade, _programs: &Programs, _textures: &Textures, _params: &DrawParameters, _font: &FontTexture) {
        unimplemented!()
    }
}