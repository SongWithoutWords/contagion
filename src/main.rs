pub mod constants;
pub mod core;
pub mod presentation;
pub mod simulation;

#[macro_use] extern crate glium;
extern crate glium_sdl2;
extern crate sdl2;
extern crate image;

use std::io::Cursor;
use std::ffi::CString;
use std::path::Path;
use glium::draw_parameters::Blend;
use glium::Surface;
use glium::index::NoIndices;
use glium::texture::texture2d::Texture2d;
use glium::VertexBuffer;
use presentation::graphics::renderer::Vertex;


struct Textures {
    zombies: Texture2d,
    police: Texture2d,
    citizen: Texture2d,
}

struct Shader_Buffers {
    zombie_shaders: (VertexBuffer<Vertex>, NoIndices),
    police_shaders: (VertexBuffer<Vertex>, NoIndices),
    citizen_shaders: (VertexBuffer<Vertex>, NoIndices),
}

fn init() -> Result<(glium_sdl2::SDL2Facade, sdl2::EventPump, Textures, glium::Program,
                     Shader_Buffers), String> {
    // TODO: initialize music

    // initialize window and eventpump
    let window_tuple = presentation::graphics::renderer::create_window();
    let mut window = window_tuple.0;
    let mut event_pump = window_tuple.1;

    // load image -> type glium::texture::texture2d::Texture2d
    let zombie_texture = presentation::graphics::renderer::load_texture(&window, "src/assets/zombie.png");
    let police_texture = presentation::graphics::renderer::load_texture(&window, "src/assets/police.png");
    let citizen_texture = presentation::graphics::renderer::load_texture(&window, "src/assets/citizen.png");
    let texture: Textures = Textures{zombies: zombie_texture, police: police_texture, citizen: citizen_texture};

    // create vertex buffer, indices
    let zombie_shader = presentation::graphics::renderer::init_shader(&window);
    let police_shader = presentation::graphics::renderer::init_shader(&window);
    let citizen_shader = presentation::graphics::renderer::init_shader(&window);
    let shader = Shader_Buffers{zombie_shaders: zombie_shader, police_shaders: police_shader, citizen_shaders: citizen_shader};

    // send vertex shader and fragment shader to glium library
    let program = glium::Program::from_source(&window, include_str!("./presentation/graphics/vs.vert"),
                                              include_str!("./presentation/graphics/fs.frag"), None).unwrap();

    Ok((window, event_pump, texture ,program, shader))
}

fn main() {
    // init
    let (mut window, mut event_pump, mut texture, mut program, mut shader) = match init() {
        // error handler if init fails
        Ok(t) => t,
        Err(err) => {
            println!("{}",err);
            std::process::exit(1);
        },
    };
    let zombie_texture = texture.zombies;
    let police_texture = texture.police;
    let citizen_texture = texture.citizen;
    let police_vb = shader.police_shaders.0;
    let police_ib = shader.police_shaders.1;
    let zombie_vb = shader.zombie_shaders.0;
    let zombie_ib = shader.zombie_shaders.1;
    let citizen_vb = shader.citizen_shaders.0;
    let citizen_ib = shader.citizen_shaders.1;

    let params = glium::DrawParameters{
        blend: Blend::alpha_blending(),
        .. Default::default()
    };

    // main game loop
    let mut running = true;
    while running {
        let mut target = window.draw();
        // draw background
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        // draw zombie
        {
            let uniforms = uniform! {
                matrix: [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [ 0.0 , 0.0, 0.0, 1.0f32],
                ],
                tex: &zombie_texture,
            };
            target.draw(&zombie_vb, &zombie_ib, &program, &uniforms,
                    &params).unwrap();
         }
        // draw police
        {
            let uniforms = uniform! {
                matrix: [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [ -0.5 , 0.0, 0.0, 1.0f32],
                ],
                tex: &police_texture,
            };
            target.draw(&police_vb, &police_ib, &program, &uniforms,
                        &params).unwrap();
        }
        // draw civilian
        {
            let uniforms = uniform! {
                matrix: [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [ 0.5 , 0.0, 0.0, 1.0f32],
                ],
                tex: &citizen_texture,
            };
            target.draw(&citizen_vb, &citizen_ib, &program, &uniforms,
                        &params).unwrap();
        }
        target.finish().unwrap();

        // Event loop: polls for events sent to all windows
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                // TODO: key inputs
                Event::Quit { .. } => {
                    running = false;
                },
                _ => ()
            }
        }
    }
}