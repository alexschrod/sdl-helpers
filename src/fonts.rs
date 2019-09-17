use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};

use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureCreator};
use sdl2::ttf::{Font as SdlFont, Sdl2TtfContext};

pub struct FontRepository<'ttf> {
    ttf_context: &'ttf Sdl2TtfContext,
    fonts: HashMap<(PathBuf, u16), Font<'ttf>>,
}

impl<'ttf> FontRepository<'ttf> {
    pub fn new(ttf_context: &'ttf Sdl2TtfContext) -> Self {
        FontRepository {
            ttf_context,
            fonts: HashMap::new(),
        }
    }

    pub fn get_font<P: AsRef<Path>, S: AsRef<str>>(
        &mut self,
        font_folder: P,
        font_name: S,
        size: u16,
    ) -> Result<&Font<'ttf>, Box<dyn Error>> {
        let font_path = PathBuf::from("fonts")
            .join(font_folder)
            .join(font_name.as_ref());

        let font_key = (font_path.clone(), size);
        let font = match self.fonts.entry(font_key) {
            Entry::Vacant(e) => {
                let font = self.ttf_context.load_font(font_path, size)?;
                e.insert(Font::new(font))
            }
            Entry::Occupied(e) => e.into_mut(),
        };
        Ok(font)
    }
}

pub struct Font<'ttf>(SdlFont<'ttf, 'static>);

impl<'ttf> Font<'ttf> {
    pub fn new(sdl_font: SdlFont<'ttf, 'static>) -> Self {
        Font(sdl_font)
    }

    #[cfg(not(feature = "unsafe_textures"))]
    pub fn render_solid_to_texture<'texture, RC, S: AsRef<str>, C: Into<Color>>(
        &self,
        texture_creator: &'texture TextureCreator<RC>,
        text: S,
        color: C,
    ) -> Result<Texture<'texture>, Box<dyn Error>> {
        let surface = self
            .0
            .render(text.as_ref())
            .solid(color)
            .map_err(|e| e.to_string())?;
        Ok(texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?)
    }

    #[cfg(not(feature = "unsafe_textures"))]
    pub fn render_shaded_to_texture<'texture, RC, S: AsRef<str>, C: Into<Color>>(
        &self,
        texture_creator: &'texture TextureCreator<RC>,
        text: S,
        color: C,
        background: C,
    ) -> Result<Texture<'texture>, Box<dyn Error>> {
        let surface = self
            .0
            .render(text.as_ref())
            .shaded(color, background)
            .map_err(|e| e.to_string())?;
        Ok(texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?)
    }

    #[cfg(not(feature = "unsafe_textures"))]
    pub fn render_blended_to_texture<'texture, RC, S: AsRef<str>, C: Into<Color>>(
        &self,
        texture_creator: &'texture TextureCreator<RC>,
        text: S,
        color: C,
    ) -> Result<Texture<'texture>, Box<dyn Error>> {
        let surface = self
            .0
            .render(text.as_ref())
            .blended(color)
            .map_err(|e| e.to_string())?;
        Ok(texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?)
    }

    #[cfg(feature = "unsafe_textures")]
    pub fn render_solid_to_texture<RC, S: AsRef<str>, C: Into<Color>>(
        &self,
        texture_creator: &TextureCreator<RC>,
        text: S,
        color: C,
    ) -> Result<Texture, Box<dyn Error>> {
        let surface = self
            .0
            .render(text.as_ref())
            .solid(color)
            .map_err(|e| e.to_string())?;
        Ok(texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?)
    }

    #[cfg(feature = "unsafe_textures")]
    pub fn render_shaded_to_texture<RC, S: AsRef<str>, C: Into<Color>>(
        &self,
        texture_creator: &TextureCreator<RC>,
        text: S,
        color: C,
        background: C,
    ) -> Result<Texture, Box<dyn Error>> {
        let surface = self
            .0
            .render(text.as_ref())
            .shaded(color, background)
            .map_err(|e| e.to_string())?;
        Ok(texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?)
    }

    #[cfg(feature = "unsafe_textures")]
    pub fn render_blended_to_texture<RC, S: AsRef<str>, C: Into<Color>>(
        &self,
        texture_creator: &TextureCreator<RC>,
        text: S,
        color: C,
    ) -> Result<Texture, Box<dyn Error>> {
        let surface = self
            .0
            .render(text.as_ref())
            .blended(color)
            .map_err(|e| e.to_string())?;
        Ok(texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?)
    }
}
