#[macro_use]
extern crate enum_map;
#[macro_use]
extern crate glium;
extern crate glium_sdl2;
extern crate image;
extern crate num;
extern crate music;
extern crate rand;
extern crate rand_xorshift;
extern crate sdl2;

use std::fs::File;
use std::time::Instant;

use glium::draw_parameters::Blend;
use glium_sdl2::SDL2Facade;
use sdl2::{EventPump, Sdl};

use crate::core::scalar:: *;
use crate::core::vector:: *;
use crate::presentation::audio::sound_effects:: *;
use crate::presentation::ui::glium_text;
use crate::game::update_scene;
use crate::scene::Scene;

pub mod constants;
pub mod core;
pub mod presentation;
pub mod simulation;
pub mod game;
pub mod scene;

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

    let mut game = game::Game::new();
    let mut scene: Option<Box<Scene>> = update_scene(&mut game, 0.0);

    let mut last_frame = Instant::now();

    // Handle the sound effects for the game
    music::start_context::<Music, TheSound, _>(&_sdl_context, 200, || {

        // Load the sound effects (bind the mp3 files with the enum)
        //load_sound_effects();

        // Play the background music until the end of the program
        //play_background();

        // main game loop
        'main_game_loop: loop {
            // Compute delta time
            let duration = last_frame.elapsed();
            let delta_time = duration.as_secs() as Scalar + 1e-9 * duration.subsec_nanos() as Scalar;
            last_frame = Instant::now();

            // update scene
            let temp_scene = scene.as_mut().unwrap().update(delta_time);
            if (!temp_scene.is_none()) || (scene.is_none()) {
                println!("changing scene");
                scene = temp_scene;
            }
            // Event loop: polls for events sent to all windows
            scene.as_mut().unwrap().handle_input(&mut event_pump, &window, delta_time);

            // render scene
            scene.as_mut().unwrap().render(&window, &programs, &textures, &params, &font);

        }
    });
}