extern crate glium;
extern crate glium_sdl2;
extern crate sdl2;
extern crate image;

use crate::constants::presentation::*;
use std::io::Cursor;
use std::ffi::CString;
use std::{thread, time};
use std::time::Instant;
use std::path::Path;

use glium::Surface;
use glium_sdl2::SDL2Facade;

use sdl2::{Sdl, EventPump};
use sdl2::event::Event;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

pub fn create_window() -> (Sdl, SDL2Facade, EventPump) {
    use glium_sdl2::DisplayBuild;
    // initialize SDL library
    let sdl_context = sdl2::init().unwrap();
    // initialize video subsystem
    let video_subsystem = sdl_context.video().unwrap();
    // OpenGL context getters and setters
    let gl_attr = video_subsystem.gl_attr();
    let mut pause_time = false;

    // setup OpenGL profile
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core); // setting type of GL context
    // Set the context into debug mode
    gl_attr.set_context_flags().debug().set();

    // creating window
    // available functionality: https://nukep.github.io/rust-sdl2/sdl2/video/struct.WindowBuilder.html#method.resizable
    let window = video_subsystem
        .window(WINDOW_TITLE, WINDOW_W, WINDOW_H)
        .resizable()
        .build_glium()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    (sdl_context, window, event_pump)
}

pub fn load_texture(window: &glium_sdl2::SDL2Facade, path: &str) -> glium::texture::texture2d::Texture2d {
    let image = image::open(Path::new(path)).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(window, image).unwrap();
    (texture)
}

use sdl2::ttf;
pub fn create_font() {
    let ttf_context = ttf::init().unwrap();
    let font = ttf_context.load_font("src/assets/ConsolaMono.ttf", 50).unwrap();
}
