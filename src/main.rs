pub mod constants;
pub mod core;
pub mod presentation;
pub mod simulation;

#[macro_use]
extern crate glium;
extern crate glium_sdl2;
extern crate sdl2;
extern crate image;
extern crate rodio;


use sdl2::{Sdl, EventPump, ttf};
use sdl2::event::Event;
use std::{io::Cursor, ffi::CString, path::Path, time::Instant};
use glium_sdl2::SDL2Facade;
use glium::draw_parameters::Blend;
use glium::Surface;
use glium::index::NoIndices;
use glium::texture::texture2d::Texture2d;
use glium::VertexBuffer;
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use std::time::Duration;
use std::io::BufReader;
use std::fs::File;
use rodio::Source;


fn init() -> Result<((Sdl, SDL2Facade, EventPump), presentation::display::Textures, glium::Program), String> {


    // Handle the Audio
    let device = rodio::default_output_device().unwrap();

    // Audio source path
    let file = File::open("src/assets/dark_rage.ogg").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    // Make this specific wav file (dark_rage.ogg) play on loop
    let source = source.take_duration(Duration::from_secs(326)).repeat_infinite();

    // Play the file
    rodio::play_raw(&device, source.convert_samples());




    // initialize window and eventpump
    let window_tuple = presentation::graphics::renderer::create_window();
    let mut window = window_tuple.1;

    // load image -> type glium::texture::texture2d::Texture2d
    let textures = presentation::display::Textures {
        zombies: presentation::graphics::renderer::load_texture(&window, "src/assets/zombie.png"),
        police: presentation::graphics::renderer::load_texture(&window, "src/assets/police.png"),
        citizen: presentation::graphics::renderer::load_texture(&window, "src/assets/citizen.png"),
    };

    // send vertex shader and fragment shader to glium library
    let program = glium::Program::from_source(&window, include_str!("./presentation/graphics/vs.vert"),
                                              include_str!("./presentation/graphics/fs.frag"), None).unwrap();
    let window_tuple: (Sdl, SDL2Facade, EventPump) = (window_tuple.0, window, window_tuple.2);

    Ok((window_tuple, textures, program))
}

fn main() {
    // init
    let (mut window_tuple, textures, program) = match init() {
        // error handler if init fails
        Ok(t) => t,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };
    let sdl_context = window_tuple.0;
    let window = window_tuple.1;
    let mut event_pump = window_tuple.2;

    let params = glium::DrawParameters {
        blend: Blend::alpha_blending(),
        ..Default::default()
    };

    let mut state = simulation::initial_state::initial_state(100);

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

        simulation::update::update(&simulation::update::UpdateArgs { dt: elapsed_t as f64 }, &mut state);

        presentation::display::display(&mut target, &window, &program, &textures, &params, &state);


        target.finish().unwrap();

        // Event loop: polls for events sent to all windows
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                // TODO: key inputs
                Event::Quit { .. } => {
                    running = false;
                }
                _ => ()
            }
        }
    }
}
