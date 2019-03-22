use crate::presentation::ui::glium_text;
use std::fs::File;
use crate::presentation::ui::glium_text::FontTexture;
use std::path::Path;
use glium_sdl2::SDL2Facade;

pub struct FontPkg {
    fonts: Vec<Font>
}

impl FontPkg {
    pub fn new() -> FontPkg {
        FontPkg {
            fonts: vec![]
        }
    }

    pub fn push(&mut self, font: Font) -> &mut FontPkg {
        self.fonts.push(font);
        self
    }

    pub fn get(&self, name: &str) -> Option<&Font> {
        for i in 0..self.fonts.len() {
            if name.to_string() == self.fonts[i].name {
                return Some(&self.fonts[i])
            }
        }
        return None
    }
}

pub struct Font {
    name: String,
    lowres: FontTexture,
    medres: FontTexture,
    highres: FontTexture,
}

impl Font {
    pub fn new(font_name: &str, path: &str, window: &SDL2Facade) -> Font {
        let font_path = Path::new(path);
        let lowres_font = glium_text::FontTexture::new(
            window,
            File::open(font_path).unwrap(),
            45,
        ).unwrap();
        let medres_font = glium_text::FontTexture::new(
            window,
            File::open(font_path).unwrap(),
            100,
        ).unwrap();
        let highres_font = glium_text::FontTexture::new(
            window,
            File::open(font_path).unwrap(),
            150,
        ).unwrap();
        Font {
            name: font_name.to_string(),
            lowres: lowres_font,
            medres: medres_font,
            highres: highres_font,
        }
    }

    pub fn lowres(&self) -> &FontTexture {
        &self.lowres
    }

    pub fn medres(&self) -> &FontTexture {
        &self.medres
    }

    pub fn highres(&self) -> &FontTexture {
        &self.highres
    }
}