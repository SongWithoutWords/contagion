pub mod constants;
pub mod core;
pub mod presentation;
pub mod simulation;

#[macro_use] extern crate glium;
extern crate glium_sdl2;
extern crate sdl2;
#[macro_use] extern crate enum_map;
extern crate image;
extern crate rand;
extern crate rand_xorshift;
extern crate rodio;
extern crate music;

use sdl2::{Sdl, EventPump};
use sdl2::keyboard::Keycode;
use std::fs::File;
use std::time::Instant;
use glium_sdl2::SDL2Facade;
use glium::draw_parameters::Blend;

use crate::core::scalar::*;
use crate::core::vector::*;
use crate::presentation::audio::sound_effects::*;
use crate::presentation::ui::glium_text;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Music {
    Background,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum The_Sound {
    Gunshot,
    Reload,
    PersonInfected,
    ZombieDeath,
}
fn init() -> Result<((Sdl, SDL2Facade, EventPump),
                     presentation::display::Textures,
                     presentation::display::Programs,
                     SoundEffectSources,
                     glium_text::FontTexture),
    String> {







    // Call the sound effects to be loaded
    let sound_effect_files = load_sound_effect_files();

    // initialize window and eventpump
    let window_tuple = presentation::graphics::renderer::create_window();
    let window = window_tuple.1;

    let font = glium_text::FontTexture::new(
        &window,
        File::open("src/assets/CONSOLA.TTF").unwrap(),
        70,
    ).unwrap();
    let textures = presentation::display::load_textures(&window);

    let programs = presentation::display::load_programs(&window);

    let window_tuple: (Sdl, SDL2Facade, EventPump) = (window_tuple.0, window, window_tuple.2);

    Ok((window_tuple, textures, programs, sound_effect_files, font))
}

pub fn play_shotgun(){
    music::play_sound(&The_Sound::Gunshot, music::Repeat::Times(1), music::MAX_VOLUME);

}
pub fn play_person_infected(){
    music::play_sound(&The_Sound::PersonInfected, music::Repeat::Times(1), music::MAX_VOLUME);

}

pub fn play_reload(){
    music::play_sound(&The_Sound::Reload, music::Repeat::Times(1), music::MAX_VOLUME);

}

pub fn play_zombie_dead(){
    music::play_sound(&The_Sound::ZombieDeath, music::Repeat::Times(1), music::MAX_VOLUME);

}

fn main() {
    // init
    let (window_tuple,
        textures,
        programs,
        sound_effect_files,
        font) = match init() {
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

    let mut state = simulation::initial_state::initial_state(100, rand::random::<u32>());
    let mut ui = presentation::ui::gui::Gui::new(presentation::ui::gui::GuiType::Selected, 0.1, 0.1, Vector2{x: -0.9,y: -0.9});
    let mut camera = presentation::camera::Camera::new();
    //  let mut audio_state = presentation::audio::sound_effects::AudioState::new();
    let mut last_frame = Instant::now();
    let mut game_paused = false;





    //let sdl = window.window.sdl_context.to_owned();

    music::start_context::<Music, The_Sound, _>(&_sdl_context,100, || {
        music::bind_music_file(Music::Background, "src/assets/dark_rage.mp3");
        music::bind_sound_file(The_Sound::Gunshot, "src/assets/gunshot.mp3");
        music::bind_sound_file(The_Sound::Reload, "src/assets/Reload.mp3");
        music::bind_sound_file(The_Sound::PersonInfected, "src/assets/person_infected.mp3");
        music::bind_sound_file(The_Sound::ZombieDeath, "src/assets/zombie_dead.mp3");
        // music::play_sound()
        music::set_volume(music::MAX_VOLUME);

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
                let sound_effects = simulation::update::update(
                    &simulation::update::UpdateArgs { dt: delta_time },
                    &mut state);
                // play_sound_effects(&sound_effect_files, &sound_effects, &mut audio_state);
            }

            let mut target = window.draw();
            presentation::display::display(&mut target, &window, &programs, &textures, &params, &state, camera_frame, &mut ui, &font);
            target.finish().unwrap();
        }

    });
}