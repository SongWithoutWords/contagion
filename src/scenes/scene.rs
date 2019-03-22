use sdl2::EventPump;
use glium_sdl2::SDL2Facade;
use crate::presentation::display::Programs;
use glium::DrawParameters;
use crate::presentation::display::Textures;
use crate::presentation::graphics::font::FontPkg;

pub enum UpdateResult {
    Exit,
    Continue,
    Transition(Box<Scene>),
}

pub trait Scene {

    fn update(&mut self,
              event_pump: &mut EventPump,
              window: &SDL2Facade,
              delta_time: f64)
              -> UpdateResult;

    fn render(&mut self,
              window: &SDL2Facade,
              programs: &Programs,
              textures: &Textures,
              params:&DrawParameters,
              fonts:&FontPkg);
}
