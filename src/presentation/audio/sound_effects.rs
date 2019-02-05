extern crate rodio;

use crate::simulation::update::SoundEffect;
use rodio::Source;

// use rodio;
use std::io;
use std::io::Read;
use std::convert::AsRef;
use std::sync::Arc;

pub struct Sound (Arc<Vec<u8>>);

impl AsRef<[u8]> for Sound {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

// Adapted from
// https://github.com/tomaka/rodio/issues/141
impl Sound {
    pub fn load(filename: &str) -> io::Result<Sound> {
        use std::fs::File;
        let mut buf = Vec::new();
        let mut file = File::open(filename)?;
        file.read_to_end(&mut buf)?;
        Ok(Sound(Arc::new(buf)))
    }
    pub fn cursor(self: &Self) -> io::Cursor<Sound> {
        io::Cursor::new(Sound(self.0.clone()))
    }
    pub fn decoder(self: &Self) -> rodio::Decoder<io::Cursor<Sound>> {
        rodio::Decoder::new(self.cursor()).unwrap()
    }
}

pub struct SoundEffectSources {
    pub gunshot: Sound,
    pub reload: Sound,
    pub person_infected: Sound,
    pub zombie_dead: Sound,
}

pub fn load_sound_effect_files() -> SoundEffectSources {

    // Loading sound files here
    SoundEffectSources {
        gunshot: Sound::load("src/assets/gunshot.ogg").unwrap(),
        reload: Sound::load("src/assets/reload.ogg").unwrap(),
        person_infected: Sound::load("src/assets/person_infected.ogg").unwrap(),
        zombie_dead: Sound::load("src/assets/zombie_dead.ogg").unwrap(),
    }
}

pub fn play_sound_effects(sources: &SoundEffectSources, sounds: &Vec<SoundEffect>) {

    // Handle the Audio
    let device = rodio::default_output_device().unwrap();

    for sound in sounds {
        let source = match sound {
            SoundEffect::Gunshot => &sources.gunshot,
            SoundEffect::PersonInfected => &sources.person_infected,
            SoundEffect::Reload => &sources.reload,
            SoundEffect::ZombieDeath => &sources.zombie_dead,
        };
        rodio::play_raw(&device, source.decoder().convert_samples());
    }
}
