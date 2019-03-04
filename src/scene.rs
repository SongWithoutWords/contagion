use sdl2::EventPump;
use glium_sdl2::SDL2Facade;
use crate::presentation::display::Programs;
use glium::DrawParameters;
use crate::presentation::display::Textures;
use crate::presentation::ui::glium_text::FontTexture;

pub trait Scene {
    fn handle_input(&mut self, event_pump:&mut EventPump, window:&SDL2Facade, delta_time: f64);
    fn update(&mut self, event_pump: &EventPump, delta_time: f64) -> Option<Box<Scene>>;
    fn render(&mut self, window:&SDL2Facade, program:&Programs, textures:&Textures, params:&DrawParameters, font:&FontTexture);
}