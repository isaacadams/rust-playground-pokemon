use image::{DynamicImage, ImageResult, RgbImage, RgbaImage};

pub struct SpriteImage {
    data: Vec<u8>,
}

impl SpriteImage {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn to_image(&self) -> ImageResult<PokemonImage> {
        let buffer = image::load_from_memory(&self.data)?;
        Ok(PokemonImage::new(buffer))
    }

    pub fn to_rgb8_image(&self) -> ImageResult<RgbImage> {
        Ok(image::load_from_memory(&self.data)?.to_rgb8())
    }
}

impl AsRef<[u8]> for SpriteImage {
    fn as_ref(&self) -> &[u8] {
        self.data.as_ref()
    }
}

pub struct PokemonImage {
    buffer: DynamicImage,
    dimensions: [usize; 2],
}

impl PokemonImage {
    pub fn new(buffer: DynamicImage) -> Self {
        Self {
            dimensions: [buffer.width() as _, buffer.height() as _],
            buffer,
        }
    }

    pub fn to_rgba8(self) -> (RgbaImage, [usize; 2]) {
        (self.buffer.to_rgba8(), self.dimensions)
    }
}
