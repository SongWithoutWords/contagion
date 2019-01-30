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
use std::collections::HashMap;

use glium::Surface;
use glium_sdl2::SDL2Facade;

use sdl2::{Sdl, EventPump};
use sdl2::event::Event;

pub fn create_window() -> (Sdl, SDL2Facade, EventPump) {
    use glium_sdl2::DisplayBuild;
    // initialize SDL library
    let sdl_context = sdl2::init().unwrap();
    // initialize video subsystem
    let video_subsystem = sdl_context.video().unwrap();
    // OpenGL context getters and setters
    let gl_attr = video_subsystem.gl_attr();

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

    let event_pump = sdl_context.event_pump().unwrap();

    (sdl_context, window, event_pump)
}

pub fn load_texture(window: &glium_sdl2::SDL2Facade, path: &str) -> glium::texture::texture2d::Texture2d {
    let image = image::open(Path::new(path)).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(window, image).unwrap();
    (texture)
}

pub fn load_texture2d_array(window: &glium_sdl2::SDL2Facade, texture_dict: HashMap<&str, u32>) -> glium::texture::Texture2dArray {
    let textures = {
        let images = texture_dict.iter().map(|(path, _)| {
            let folder = "src/assets/".to_string();
            let path = folder + &path;
            let image = image::open(Path::new(&path)).unwrap().to_rgba();
            let image_dimensions = image.dimensions();
            glium::texture::RawImage2d::from_raw_rgba(image.into_raw(), image_dimensions)
        }).collect::<Vec<_>>();

        glium::texture::Texture2dArray::new(window, images).unwrap()
    };
    (textures)
}


#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2]
}

pub fn init_shader(window: &glium_sdl2::SDL2Facade) -> (glium::VertexBuffer<Vertex>, glium::index::NoIndices){
    implement_vertex!(Vertex, position, tex_coords);
    // 0      1
    // +------+
    // |    / |
    // |  /   |
    // |/     |
    // +------+
    // 2      3
    let vertex0 = Vertex { position: [-0.5, 0.5], tex_coords: [0.0, 1.0]};
    let vertex1 = Vertex { position: [0.5, 0.5], tex_coords: [1.0, 1.0]};
    let vertex2 = Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0]};
    let vertex3 = Vertex { position: [0.5, -0.5], tex_coords: [1.0, 0.0]};
    let shape = vec![vertex0, vertex1, vertex2, vertex1, vertex3, vertex2];

    let vertex_buffer = glium::VertexBuffer::new(window, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    (vertex_buffer,indices)
}

#[derive(Copy, Clone)]
pub struct Vertex2dArray {
    i_position: [f32; 2],
    i_tex_id: u32,
}

pub fn from_texture2d_array(window: &glium_sdl2::SDL2Facade, textures: glium::texture::Texture2dArray, texture_dict: HashMap<&str, u32>) ->
                                                        (glium::VertexBuffer<Vertex2dArray>, Vec<u16>){
    implement_vertex!(Vertex, i_position, i_tex_id);
    // initializing empty vertex buffer with element size SPRITES_COUNT * 4 since there are 4 vertices per sprite
    let mut vb: glium::VertexBuffer<Vertex> = glium::VertexBuffer::empty_dynamic(window,
                                                                                 SPRITES_COUNT * 4).unwrap();
    // initializing empty index buffer. You multiply 6, since to draw a rectangle you would need to draw 6 sides
    // 0--> 1 --> 2 --> 1 --> 3 --> 2
    // 0      1
    // +------+
    // |    / |
    // |  /   |
    // |/     |
    // +------+
    // 2      3
    let mut ib_data = Vec::with_capacity(SPRITES_COUNT * 6);
    for (num, sprite) in vb.map().chunks_mut(4).enumerate() {
        let tex_id = match value {
            1 => texture_dict.get("zombie.png").unwrap(),
        };
        // vertex 0
        sprite[0].i_position[0] = -0.5; // x
        sprite[0].i_position[1] = 0.5; // y
        sprite[0].i_tex_id = tex_id.clone();
        // vertex 1
        sprite[1].i_position[0] = 0.5; // x
        sprite[1].i_position[1] = 0.5; // y
        sprite[1].i_tex_id = tex_id.clone();
        // vertex 2
        sprite[2].i_position[0] = -0.5; // x
        sprite[2].i_position[1] = -0.5; // y
        sprite[2].i_tex_id = tex_id.clone();
        // vertex 3
        sprite[3].i_position[0] = 0.5; // x
        sprite[3].i_position[1] = -0.5; // y
        sprite[3].i_tex_id = tex_id.clone();

        let num = num as u16;
        ib_data.push(num * 4);      // 0 -->
        ib_data.push(num * 4 + 1);  // 1 -->
        ib_data.push(num * 4 + 2);  // 2 -->
        ib_data.push(num * 4 + 1);  // 1 -->
        ib_data.push(num * 4 + 3);  // 3 -->
        ib_data.push(num * 4 + 2);  // 2
    }

    (vb,ib_data)
}

use sdl2::ttf;
pub fn create_font() {
    let ttf_context = ttf::init().unwrap();
    let font = ttf_context.load_font("src/assets/ConsolaMono.ttf", 50).unwrap();
}