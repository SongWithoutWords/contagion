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

use crate::core::scalar::*;
use crate::core::vector::*;
use crate::presentation::ui::glium_text;

pub mod constants;
pub mod core;
pub mod presentation;
pub mod simulation;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Music {
    Background,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum TheSound {
    Gunshot,
    Reload,
    PersonInfected,
    ZombieDeath,
}

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
        File::open("src/assets/CONSOLA.TTF").unwrap(),
        70,
    ).unwrap();
    let textures = presentation::display::load_textures(&window);

    let programs = presentation::display::load_programs(&window);

    let window_tuple: (Sdl, SDL2Facade, EventPump) = (window_tuple.0, window, window_tuple.2);

    Ok((window_tuple, textures, programs, font))
}

// Plays the shotgun sound once every time it is called
pub fn play_shotgun() {
    music::play_sound(&TheSound::Gunshot, music::Repeat::Times(1), music::MAX_VOLUME);
}

// Plays the person_infected sound once every time it is called
pub fn play_person_infected() {
    music::play_sound(&TheSound::PersonInfected, music::Repeat::Times(1), music::MAX_VOLUME);
}

// Plays the reload sound once every time it is called
pub fn play_reload() {
    music::play_sound(&TheSound::Reload, music::Repeat::Times(1), music::MAX_VOLUME);
}

// Plays the dead zombie sound once every time it is called
pub fn play_zombie_dead() {
    music::play_sound(&TheSound::ZombieDeath, music::Repeat::Times(1), music::MAX_VOLUME);
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

    let mut ui = presentation::ui::gui::Gui::new(presentation::ui::gui::GuiType::Selected, 0.1, 0.1, Vector2 { x: -0.9, y: -0.9 });
    let mut camera = presentation::camera::Camera::new();

    let mut component = presentation::ui::gui::Component::init_demo();
    let mut camera = presentation::camera::Camera::new();

    let mut last_frame = Instant::now();
    let mut game_paused = false;


    // Handle the sound effects for the game
    music::start_context::<Music, TheSound, _>(&_sdl_context, 200, || {

        // Bind the enum variables with the actual mp3 files
        music::bind_music_file(Music::Background, "src/assets/dark_rage.mp3");
        music::bind_sound_file(TheSound::Gunshot, "src/assets/gunshot.mp3");
        music::bind_sound_file(TheSound::Reload, "src/assets/Reload.mp3");
        music::bind_sound_file(TheSound::PersonInfected, "src/assets/person_infected.mp3");
        music::bind_sound_file(TheSound::ZombieDeath, "src/assets/zombie_dead.mp3");

        // Sets the volume of the sounds
        music::set_volume(music::MAX_VOLUME);

        // Play the Background sound
        music::play_music(&Music::Background, music::Repeat::Forever);


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
                    }
                    Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                        game_paused = !game_paused;
                    }
                    Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                        println!("Debug info:");
                        println!("  DT:               {:?}", delta_time);
                        println!("  FPS:              {:?}", 1.0 / delta_time);
                        println!("  Entity count:     {:?}", state.entities.len());
                        println!("  Projectile count: {:?}", state.projectiles.len());
                    }
                    Event::MouseButtonDown { timestamp: _, window_id: _, which: _, mouse_btn, x, y } => {
                        use sdl2::mouse::MouseButton;
                        match mouse_btn {
                            MouseButton::Left { .. } => {
                                simulation::control::update_selected(0, &mut state, &window, camera_frame, x, y);
                                for i in 0..state.is_selected.len() {

                                    if state.is_selected[i] == true {

                                    if state.is_selected[i] {

                                        println!("selected: {:?}", state.is_selected[i]);
                                    }
                                }
                            }
                            MouseButton::Right { .. } => {
                                simulation::control::issue_police_order(simulation::control::PoliceOrder::Move, &mut state, &window, camera_frame, x, y);
                            }
                            _ => ()
                        }
                    }
                    Event::MouseWheel { timestamp: _, window_id: _, which: _, x: _, y, direction: _ } => {
                        camera.set_zoom(y);
                    }
                    _ => ()
                }
            }

            if !game_paused {
                let _not_paused_game = simulation::update::update(
                    &simulation::update::UpdateArgs { dt: delta_time },
                    &mut state);
            }


            let mut target = window.draw();
            presentation::display::display(&mut target, &window, &programs, &textures, &params, &state, camera_frame, &mut ui, &font);
            target.finish().unwrap();
        }
    });
}

        let mut target = window.draw();
        presentation::display::display(&mut target, &window, &programs, &textures, &params, &state, camera_frame, &mut component, &font);
        target.finish().unwrap();
    }
}

