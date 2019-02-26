#[macro_use]
extern crate enum_map;
#[macro_use]
extern crate glium;
extern crate glium_sdl2;
extern crate image;
extern crate music;
extern crate rand;
extern crate rand_xorshift;
extern crate sdl2;

use std::fs::File;
use std::time::Instant;

use glium::draw_parameters::Blend;
use glium_sdl2::SDL2Facade;
use sdl2::{EventPump, Sdl};
use sdl2::keyboard::Keycode;
//use sdl2::mouse::MouseButton;

use crate::core::scalar:: *;
use crate::core::vector:: *;
use crate::presentation::audio::sound_effects:: *;
use crate::presentation::ui::glium_text;
use crate::presentation::ui::gui::{CURRENT,ActiveWindow};

pub mod constants;
pub mod core;
pub mod presentation;
pub mod simulation;

fn init() -> Result<((Sdl, SDL2Facade, EventPump),
                     presentation::display::Textures,
                     presentation::display::Programs,
                     glium_text::FontTexture),
    String> {

    // initialize window and eventpump
    let window_tuple = presentation::graphics::renderer::create_window();
    let window = window_tuple.1;

    let font = glium_text::FontTexture::new(
        &window,
        File::open("assets/fonts/consola.ttf").unwrap(),
        30,
    ).unwrap();
    let textures = presentation::display::load_textures(&window);

    let programs = presentation::display::load_programs(&window);

    let window_tuple: (Sdl, SDL2Facade, EventPump) = (window_tuple.0, window, window_tuple.2);

    Ok((window_tuple, textures, programs, font))
}


fn main() {
    // init
    let (window_tuple,
        textures,
        programs,
        font) = match init() {
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

    let mut state = simulation::initial_state::initial_state(100, rand::random::<u32>());
    let mut ui = presentation::ui::gui::Component::init_demo();
    let mut camera = presentation::camera::Camera::new();
    let mut control = simulation::control::Control::new();

    let mut last_frame = Instant::now();
    let mut game_paused = false;
    let mut terminate = false;


    // Handle the sound effects for the game
    music::start_context::<Music, TheSound, _>(&_sdl_context, 200, || {

        // Load the sound effects (bind the mp3 files with the enum)
        load_sound_effects();

        // Play the background music until the end of the program
        play_background();

        // main game loop
        'main_game_loop: loop {
            if terminate {
                break 'main_game_loop
            }
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
                    // TODO: refactor into control
                    // Exit window if escape key pressed or quit event triggered
                    Event::Quit { .. } => {
                        break 'main_game_loop;
                    }
                    Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                        game_paused = !game_paused;
                    },
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        unsafe {
                            if CURRENT == ActiveWindow::Menu {
                                CURRENT = ActiveWindow::Game;
                                game_paused = !game_paused;
                            }
                            else if CURRENT == ActiveWindow::Instruction {
                                CURRENT = ActiveWindow::Menu;
                            }
                            else if CURRENT == ActiveWindow::Game {
                                CURRENT = ActiveWindow::Menu;
                                game_paused = !game_paused;
                            }
                        }

                    },
                    Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                        println!("Debug info:");
                        println!("  DT:               {:?}", delta_time);
                        println!("  FPS:              {:?}", 1.0 / delta_time);
                        println!("  Entity count:     {:?}", state.entities.len());
                        println!("  Projectile count: {:?}", state.projectiles.len());
                    }
                    Event::MouseWheel { timestamp: _, window_id: _, which: _, x: _, y, direction: _ } => {
                        camera.set_zoom(y);
                    }
                    _ => {
                        ui.handle_event(event, &mut control, &window, camera_frame, &mut state, &mut game_paused, &mut terminate);
                    }
                }
            }

            if !game_paused {
                let _not_paused_game = simulation::update::update(
                    &simulation::update::UpdateArgs { dt: delta_time },
                    &mut state);
            }

            let mut target = window.draw();
            presentation::display::display(&mut target, &window, &programs, &textures, &params, &state, camera_frame, &mut ui, &font, &control);
            target.finish().unwrap();
        }
    });
}
