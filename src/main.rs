pub mod constants;
pub mod core;
pub mod presentation;
pub mod simulation;

#[macro_use] extern crate glium;
extern crate glium_sdl2;
extern crate sdl2;
#[macro_use] extern crate enum_map;
extern crate image;
extern crate rodio;

use sdl2::{Sdl, EventPump};
use sdl2::keyboard::Keycode;
use std::time::Instant;
use glium_sdl2::SDL2Facade;
use glium::draw_parameters::Blend;
use std::time::Duration;
use std::io::BufReader;
use std::fs::File;
use rodio::Source;

use crate::core::scalar::*;
use crate::core::vector::*;
use crate::presentation::audio::sound_effects::*;


fn init() -> Result<((Sdl, SDL2Facade, EventPump),
                     presentation::display::Textures,
                     glium::Program,
                     SoundEffectSources),
                    String> {

    // Handle the Audio
    let device = rodio::default_output_device().unwrap();

    // Call the sound effects to be loaded
    let sound_effect_files = load_sound_effect_files();

    // Background audio source path
    let file = File::open("src/assets/dark_rage.ogg").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    // Make this specific ogg file (dark_rage.ogg) play on loop
    let source = source.take_duration(Duration::from_secs(326)).repeat_infinite();

    // Play the file
    rodio::play_raw(&device, source.convert_samples());

    // initialize window and eventpump
    let window_tuple = presentation::graphics::renderer::create_window();
    let window = window_tuple.1;

    let textures = presentation::display::load_textures(&window);

    // send vertex shader and fragment shader to glium library
    let program = glium::Program::from_source(&window, include_str!("./presentation/graphics/vs.vert"),
                                              include_str!("./presentation/graphics/fs.frag"), None).unwrap();
    let window_tuple: (Sdl, SDL2Facade, EventPump) = (window_tuple.0, window, window_tuple.2);

    Ok((window_tuple, textures, program, sound_effect_files))
}

fn main() {
    // init
    let (window_tuple, textures, program, _sound_effect_files) = match init() {
        // error handler if init fails
        Ok(t) => t,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };
    let _sdl_context = window_tuple.0;
    let window = window_tuple.1;
    let mut event_pump = window_tuple.2;
    let params = glium::DrawParameters {
        blend: Blend::alpha_blending(),
        ..Default::default()
    };
    let mut state = simulation::initial_state::initial_state(20);
    let mut camera = presentation::camera::Camera::new();
    let mut last_frame = Instant::now();
    let mut game_paused = false;

    // main game loop
    'main_game_loop: loop {
        // Compute delta time
        let duration = last_frame.elapsed();
        let delta_time = duration.as_secs() as Scalar + 1e-9 * duration.subsec_nanos() as Scalar;
        last_frame = Instant::now();
        let keyboard_state = event_pump.keyboard_state();

        camera.update(&keyboard_state, delta_time);

        let camera_frame = camera.compute_matrix();

        // Event loop: polls for events sent to all windows
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                // Exit window if escape key pressed or quit event triggered
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main_game_loop;
                },
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    game_paused = ! game_paused;
                },
                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                    println!("Debug info:");
                    println!("  DT:               {:?}", delta_time);
                    println!("  FPS:              {:?}", 1.0 / delta_time);
                    println!("  Entity count:     {:?}", state.entities.len());
                    println!("  Projectile count: {:?}", state.projectiles.len());
                },
                Event::MouseButtonDown {timestamp: _, window_id: _, which: _, mouse_btn, x, y } => {
                    use sdl2::mouse::MouseButton;
                    match mouse_btn {
                        MouseButton::Left { .. } => {
                            simulation::control::update_selected(0, &mut state, &window, camera_frame, x, y);
                                for i in 0..state.is_selected.len() {
                                    if state.is_selected[i] == true {
                                    println!("selected: {:?}", state.is_selected[i]);
                                    }
                                }
                        }
                        MouseButton::Right { .. } => {
                            simulation::control::issue_police_order(simulation::control::PoliceOrder::Move, &mut state, &window, camera_frame, x, y);
                        }
                        _ => ()
                    }
                },
                Event::MouseWheel {timestamp: _, window_id: _, which: _, x: _, y, direction: _} => {
                    camera.set_zoom(y);
                }
                _ => ()
            }
        }

        if !game_paused {
            let _sound_effects = simulation::update::update(
                &simulation::update::UpdateArgs { dt: delta_time },
                &mut state);
            // Sound effects temporarily disabled because they are not working
            // play_sound_effects(&sound_effect_files, &sound_effects);
        }

        let mut target = window.draw();
        presentation::display::display(&mut target, &window, &program, &textures, &params, &state, camera_frame);
        target.finish().unwrap();
    }
}
