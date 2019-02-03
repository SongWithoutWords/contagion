extern crate rodio;

use crate::simulation::update::SoundEffect;
use std::time::Duration;
use std::io::BufReader;
use std::fs::File;
use rodio::Source;

pub struct SoundEffectFiles {
    // Storing sound files here
    gunshot_file: File,
    person_infected_file: File,
    reload_file: File,
    zombie_dead_file: File,
}

pub fn load_sound_effect_files() -> SoundEffectFiles {

    // Loading sound files here
    SoundEffectFiles {
        gunshot_file: File::open("src/assets/gunshot.wav").unwrap(),
        person_infected_file: File::open("src/assets/person_infected.wav").unwrap(),
        reload_file: File::open("src/assets/reload.wav.ogg").unwrap(),
        zombie_dead_file: File::open("src/assets/zombie_dead.ogg").unwrap(),
    }
}

pub fn play_sound_effects(sound_effect_files: &SoundEffectFiles, sounds: &Vec<SoundEffect>) {

    // Handle the Audio
    let device = rodio::default_output_device().unwrap();

    // Read the sounds from the files
    let gunshot_read = rodio::Decoder::new(BufReader::new(sound_effect_files.gunshot_file)).unwrap();
    let person_infected_read = rodio::Decoder::new(BufReader::new(sound_effect_files.person_infected_file)).unwrap();
    let reload_read = rodio::Decoder::new(BufReader::new(sound_effect_files.reload_file)).unwrap();
    let zombie_dead_read = rodio::Decoder::new(BufReader::new(sound_effect_files.zombie_dead_file)).unwrap();

    // Make the sounds start from the beginning and available to be used.
    let gunshot_sound = gunshot_read.take_duration(Duration::from_secs(0));
    let person_infected_sound = person_infected_read.take_duration(Duration::from_secs(0));
    let reload_sound = reload_read.take_duration(Duration::from_secs(0));
    let zombie_dead_sound = zombie_dead_read.take_duration(Duration::from_secs(0));


    for sound in sounds {
        match sound {
            // Play the gunshot sound
            SoundEffect::Gunshot =>
                rodio::play_raw(&device, gunshot_sound.convert_samples()),

            // Play the infected person sound
            SoundEffect::PersonInfected =>
                rodio::play_raw(&device, person_infected_sound.convert_samples()),

            // Play the reload sound
            SoundEffect::Reload =>
                rodio::play_raw(&device, reload_sound.convert_samples()),

            // Play the dead zombie sound
            SoundEffect::ZombieDeath =>
                rodio::play_raw(&device, zombie_dead_sound.convert_samples())
        }
    }
}
