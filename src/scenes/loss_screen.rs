use crate::simulation::state::State;
use crate::simulation::game_state::GameState;
use crate::presentation::ui::gui::Component;
use crate::scenes::scene::{Scene, UpdateResult};
use sdl2::EventPump;
use glium_sdl2::SDL2Facade;
use crate::presentation::display::{Programs, Textures};
use glium::DrawParameters;
use crate::presentation::ui::glium_text::FontTexture;

pub struct LossScreen {
    _state: State,
    _game_state: GameState,
    _gui: Component,
}

impl Scene for LossScreen {
    fn update(&mut self, _event_pump: &mut EventPump, _window: &SDL2Facade, _delta_time: f64) -> UpdateResult {
        unimplemented!()
    }

    fn render(&mut self, _window: &SDL2Facade, _programs: &Programs, _textures: &Textures, _params: &DrawParameters, _font: &FontTexture) {
        unimplemented!()
    }
}