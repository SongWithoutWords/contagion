extern crate rodio;

use crate::simulation::update::SoundEffect;
use std::time::Duration;
use std::io::BufReader;
use std::fs::File;
use rodio::Source;

pub fn play_sound_effects(sounds: &Vec<SoundEffect>) {

    // Handle the Audio
    let device = rodio::default_output_device().unwrap();

    // Import the sounds from their files
    let gunshot_file = File::open("src/assets/gunshot.wav").unwrap();
    let person_infected_file = File::open("src/assets/person_infected.wav").unwrap();
    let reload_file = File::open("src/assets/reload.wav.ogg").unwrap();
    let zombie_dead_file = File::open("src/assets/zombie_dead.ogg").unwrap();

    // Read the sounds from the files
    let gunshot_read = rodio::Decoder::new(BufReader::new(gs)).unwrap();
    let person_infected_read = rodio::Decoder::new(BufReader::new(pi)).unwrap();
    let reload_read = rodio::Decoder::new(BufReader::new(r)).unwrap();
    let zombie_dead_read = rodio::Decoder::new(BufReader::new(zd)).unwrap();

    // Make the sounds start from the beginning and available to be used.
    let gunshot_sound = gunshot_read.take_duration(Duration::from_secs(0));
    let person_infected_sound = person_infected_read.take_duration(Duration::from_secs(0));
    let reload_sound = reload_read.take_duration(Duration::from_secs(0));
    let zombie_dead_sound = zombie_dead_read.take_duration(Duration::from_secs(0));

    // Play the file


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
