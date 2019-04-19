use crate::core::vector::*;
use crate::presentation::camera::Camera;
use crate::simulation::update::{Sound, SoundType};


#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Music {
    Background,
}

const AUDIO_ENABLED: bool = !cfg!(target_os = "macos");


pub fn load_sound_effects(){
    // Bind the enum variables with the actual mp3 files
    if AUDIO_ENABLED {
        music::bind_music_file(Music::Background, "assets/audio/music/dark_rage.mp3");
        music::bind_sound_file(SoundType::GunshotHandgun, "assets/audio/sfx/gunshot_handgun.mp3");
        music::bind_sound_file(SoundType::GunshotRifle, "assets/audio/sfx/gunshot_rifle.wav");
        music::bind_sound_file(SoundType::Reload, "assets/audio/sfx/reload.mp3");
        music::bind_sound_file(SoundType::PersonInfected, "assets/audio/sfx/person_infected.mp3");
        music::bind_sound_file(SoundType::ZombieDeath, "assets/audio/sfx/zombie_dead.mp3");
    }
}

// Plays the background music until the end of the program
pub fn play_background() {
    if AUDIO_ENABLED {
        music::play_music(&Music::Background, music::Repeat::Forever);
    }
}

pub fn play_sounds(sounds: &Vec<Sound>, camera: &Camera) {
    if AUDIO_ENABLED {
        for sound in sounds {

            let camera_position = camera.get_world_position();
            let camera_zoom = 0.5 * (camera.zoom.x + camera.zoom.y);

            let distance = vector3(
                sound.position.x - camera_position.x,
                sound.position.y - camera_position.y,
                0.1 / camera_zoom).length();

            let intensity = 2.0 / (1.0 + distance.sqrt());

            music::play_sound(&sound.sound_type, music::Repeat::Times(0), intensity);
        }
    }
}
