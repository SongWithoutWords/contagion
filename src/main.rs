pub mod constants;
pub mod core;
pub mod presentation;
pub mod simulation;

#[macro_use] extern crate glium;
extern crate glium_sdl2;
extern crate sdl2;
extern crate image;

use sdl2::{Sdl, EventPump, ttf};
use sdl2::event::Event;
use std::{io::Cursor, ffi::CString, path::Path, time::Instant};
use std::collections::HashMap;
use glium_sdl2::SDL2Facade;
use glium::draw_parameters::Blend;
use glium::Surface;
use glium::index::NoIndices;
use glium::texture::{texture2d::Texture2d, Texture2dArray};
use glium::VertexBuffer;
use presentation::graphics::renderer::Vertex;

struct Textures {
    zombies: Texture2d,
    police: Texture2d,
    citizen: Texture2d,
}

struct ShaderBuffers {
    zombie_shaders: (VertexBuffer<Vertex>, NoIndices),
    police_shaders: (VertexBuffer<Vertex>, NoIndices),
    citizen_shaders: (VertexBuffer<Vertex>, NoIndices),
}

fn init() -> Result<((Sdl, SDL2Facade, EventPump), Textures, glium::Program,
                     ShaderBuffers), String> {
    // TODO: initialize music

    // initialize window and eventpump
    let window_tuple = presentation::graphics::renderer::create_window();
    let window = window_tuple.1;

    // load image -> type glium::texture::texture2d::Texture2d
    let zombie_texture: Texture2d = presentation::graphics::renderer::load_texture(&window, "src/assets/zombie.png");
    let police_texture: Texture2d = presentation::graphics::renderer::load_texture(&window, "src/assets/police.png");
    let citizen_texture: Texture2d = presentation::graphics::renderer::load_texture(&window, "src/assets/citizen.png");
    let texture: Textures = Textures{zombies: zombie_texture, police: police_texture, citizen: citizen_texture};

    // sprite batching
    let texture_dict: HashMap<&str, u32> = {
        let mut map: HashMap<&str, u32> = HashMap::new();
        let paths = vec!["zombie.png"];
        for (num, path) in (0u32..).zip(paths.into_iter()) {
            map.insert(path, num);
        }
        map
    };
    let textures2array: Texture2dArray = presentation::graphics::renderer::load_texture2d_array(&window, texture_dict);

    // create vertex buffer, indices
    let zombie_shader = presentation::graphics::renderer::init_shader(&window);
    let police_shader = presentation::graphics::renderer::init_shader(&window);
    let citizen_shader = presentation::graphics::renderer::init_shader(&window);
    let shader = ShaderBuffers {zombie_shaders: zombie_shader, police_shaders: police_shader, citizen_shaders: citizen_shader};

    // send vertex shader and fragment shader to glium library
    let program = glium::Program::from_source(&window, include_str!("./presentation/graphics/vs.vert"),
                                              include_str!("./presentation/graphics/fs.frag"), None).unwrap();
    let window_tuple: (Sdl, SDL2Facade, EventPump) = (window_tuple.0, window, window_tuple.2);

    Ok((window_tuple, texture ,program, shader))
}

fn main() {
    // init
    let (window_tuple, texture, program, shader) = match init() {
        // error handler if init fails
        Ok(t) => t,
        Err(err) => {
            println!("{}",err);
            std::process::exit(1);
        },
    };
    let sdl_context = window_tuple.0;
    let window = window_tuple.1;
    let mut event_pump = window_tuple.2;

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

    let mut last_frame = Instant::now();
    let mut last_second = Instant::now();
    let mut fps = 0;
    let mut elapsed_t;

    // main game loop
    let mut running = true;
    while running {
        // Handle FPS
        {
            // use elapsed_t for transforming matrices
            let dt = last_frame.elapsed().subsec_nanos() as f32 / 1.0e6; // ns -> ms
            elapsed_t = dt / 1.0e3; // ms -> s
            last_frame = Instant::now();
            fps += 1;
            if last_frame.duration_since(last_second).as_secs() >= 1 {
                println!("FPS: {:?}", fps);
                last_second = Instant::now();
                fps = 0;
            }
        }

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