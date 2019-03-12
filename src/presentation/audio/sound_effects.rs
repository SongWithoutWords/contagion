extern crate music;

use crate::simulation::update::Sound;


#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Music {
    Background,
}

const AUDIO_ENABLED: bool = !cfg!(target_os = "macos");


pub fn load_sound_effects(){
        // Bind the enum variables with the actual mp3 files
    if AUDIO_ENABLED {
        music::bind_music_file(Music::Background, "assets/audio/music/dark_rage.mp3");
        music::bind_sound_file(Sound::Gunshot, "assets/audio/sfx/gunshot.mp3");
        music::bind_sound_file(Sound::Reload, "assets/audio/sfx/reload.mp3");
        music::bind_sound_file(Sound::PersonInfected, "assets/audio/sfx/person_infected.mp3");
        music::bind_sound_file(Sound::ZombieDeath, "assets/audio/sfx/zombie_dead.mp3");
    }
}

// Plays the background music until the end of the program
pub fn play_background() {
    if AUDIO_ENABLED {
        music::play_music(&Music::Background, music::Repeat::Forever);
    }
}

pub fn play_sounds(sounds: &Vec<Sound>) {
    if AUDIO_ENABLED {
        for sound in sounds {
            music::play_sound(sound, music::Repeat::Times(0), 0.5);
        }
    }
}
