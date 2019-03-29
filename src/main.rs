#[macro_use]
extern crate enum_map;
#[macro_use]
extern crate glium;
extern crate glium_sdl2;
extern crate image;
extern crate lerp;
extern crate num;
extern crate music;
extern crate rand;
extern crate rand_xorshift;
extern crate sdl2;

use std::time::Instant;

use glium::draw_parameters::Blend;
use glium_sdl2::SDL2Facade;
use sdl2::{EventPump, Sdl};

use crate::core::scalar:: *;
use crate::core::vector:: *;
use crate::presentation::audio::sound_effects:: *;
use crate::scenes::main_menu;
use crate::scenes::scene::{Scene, UpdateResult};
use crate::simulation::update::Sound;
use crate::presentation::graphics::font::{Font, FontPkg};

pub mod constants;
pub mod core;
pub mod presentation;
pub mod simulation;
pub mod scenes;

fn init() -> Result<((Sdl, SDL2Facade, EventPump),
                     presentation::display::Textures,
                     presentation::display::Programs,
                     FontPkg),
    String> {

    // initialize window and eventpump
    let window_tuple = presentation::graphics::renderer::create_window();
    let window = window_tuple.1;

    // initialize fonts and package
    let consola_font = Font::new("Consola", "assets/fonts/consola.ttf", &window);
    let mut fonts = FontPkg::new();
    fonts.push(consola_font);

    let textures = presentation::display::load_textures(&window);

    let programs = presentation::display::load_programs(&window);

    let window_tuple: (Sdl, SDL2Facade, EventPump) = (window_tuple.0, window, window_tuple.2);

    Ok((window_tuple, textures, programs, fonts))
}


fn main() {
    // init
    let (window_tuple,
        textures,
        programs,
        fonts) = match init() {
        Ok(t) => t,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };
    let _sdl_context = window_tuple.0;
    let mut window = window_tuple.1;
    let mut event_pump = window_tuple.2;
    let params = glium::DrawParameters {
        blend: Blend::alpha_blending(),
        ..Default::default()
    };

    let mut scene: Box<Scene> = Box::new(main_menu::MainMenu::new());
    let mut last_frame = Instant::now();

    // Handle the sound effects for the game
    music::start_context::<Music, Sound, _>(&_sdl_context, 200, || {

        load_sound_effects();

        play_background();

        'main_game_loop: loop {
            // Compute delta time
            let duration = last_frame.elapsed();
            let delta_time = duration.as_secs() as Scalar + 1e-9 * duration.subsec_nanos() as Scalar;
            last_frame = Instant::now();

            let opt_next_scene = scene.update(&mut event_pump, &mut window, delta_time);

            match opt_next_scene {
                UpdateResult::Exit => break,
                UpdateResult::Continue => (),
                UpdateResult::Transition(next_scene) => scene = next_scene,
            };

            scene.render(&window, &programs, &textures, &params, &fonts);
        }
    });
}
