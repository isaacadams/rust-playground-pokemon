use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::sprite::SpriteImage;

#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonResult {
    name: String,
    id: u32,
    sprites: Sprites,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sprites {
    #[serde(rename(deserialize = "front_default"))]
    front: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamedAPIResource {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamedAPIResourceList {
    pub count: usize,
    pub results: Vec<NamedAPIResource>,
}

impl PokemonResult {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn print_found_in_wild(&self) {
        println!("encountered a wild {} {}!", self.id, self.name);
    }

    pub fn fetch_sprite(&self) -> Result<SpriteImage, Box<dyn Error>> {
        let bytes = reqwest::blocking::get(&self.sprites.front)?.bytes()?;
        let sprite = SpriteImage::new(bytes.to_vec());
        Ok(sprite)
    }
}
