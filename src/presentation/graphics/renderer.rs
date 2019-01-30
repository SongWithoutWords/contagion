extern crate glium;
extern crate glium_sdl2;
extern crate sdl2;
extern crate image;

use std::io::Cursor;
use std::ffi::CString;
use std::{thread, time};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::time::Instant;
use std::path::Path;

use glium::Surface;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

pub fn create_window() -> (glium_sdl2::SDL2Facade, sdl2::EventPump) {
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
        .window("Contagion", WIDTH, HEIGHT)
        .resizable()
        .build_glium()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    (window, event_pump)
}

pub fn load_texture(window: &glium_sdl2::SDL2Facade, path: &str) -> glium::texture::texture2d::Texture2d {
    let image = image::open(Path::new(path)).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(window, image).unwrap();
    (texture)
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

pub fn init_shader(window: &glium_sdl2::SDL2Facade) -> (glium::VertexBuffer<Vertex>, glium::index::NoIndices){
    implement_vertex!(Vertex, position, tex_coords);
    // 1      2
    // +------+
    // |    / |
    // |  /   |
    // |/     |
    // +------+
    // 3      4
    let vertex1 = Vertex { position: [-0.5, 0.5], tex_coords: [0.0, 1.0] };
    let vertex2 = Vertex { position: [0.5, 0.5], tex_coords: [1.0, 1.0] };
    let vertex3 = Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] };
    let vertex4 = Vertex { position: [0.5, 0.5], tex_coords: [1.0, 1.0] };
    let vertex5 = Vertex { position: [0.5, -0.5], tex_coords: [1.0, 0.0] };
    let vertex6 = Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] };
    let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];

    let vertex_buffer = glium::VertexBuffer::new(window, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    (vertex_buffer,indices)
}
