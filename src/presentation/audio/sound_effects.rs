extern crate rodio;

use std::convert::AsRef;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;
use std::sync::Arc;
use std::time::Duration;

use enum_map::EnumMap;
use rodio::Sink;
use rodio::Source;

use crate::simulation::update::SoundEffect;

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

// this is the maximum number of concurrent sound effects
pub const SOUND_EFFECT_SINK_COUNT: usize = 8;
pub struct AudioState {
    device: rodio::Device,
    ambient_sink: Sink,
    sound_effect_sinks: Vec<Sink>,
    index_of_least_recently_used: usize,
}
impl AudioState {
    pub fn new() -> Self {
        let device = rodio::default_output_device().unwrap();

        let ambient_file = File::open("src/assets/dark_rage.ogg").unwrap();
        let ambient_source = rodio::Decoder::new(BufReader::new(ambient_file)).unwrap();
        let ambient_sink = Sink::new(&device);
        ambient_sink.append(ambient_source.repeat_infinite());

        let sinks = (0..SOUND_EFFECT_SINK_COUNT).map(|_| Sink::new(&device)).collect();

        AudioState {
            device: device,
            ambient_sink: ambient_sink,
            sound_effect_sinks: sinks,
            index_of_least_recently_used: 0,
        }
    }
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

pub fn play_sound_effects(
    sources: &SoundEffectSources,
    sounds: &Vec<SoundEffect>,
    state: &mut AudioState
) {
    for sound in sounds {

        let source = match sound {
            SoundEffect::Gunshot => &sources.gunshot,
            SoundEffect::PersonInfected => &sources.person_infected,
            SoundEffect::Reload => &sources.reload,
            SoundEffect::ZombieDeath => &sources.zombie_dead,
        };

        state.sound_effect_sinks[state.index_of_least_recently_used] = Sink::new(&state.device);
        state.sound_effect_sinks[state.index_of_least_recently_used].append(source.decoder());

        state.index_of_least_recently_used += 1;
        state.index_of_least_recently_used %= SOUND_EFFECT_SINK_COUNT;
    }
}
