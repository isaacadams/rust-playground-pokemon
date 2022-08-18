use hyper::body::Bytes;
use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;

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

pub fn get_pokemon_data(no: u32) -> Result<PokemonResult, reqwest::Error> {
    let response = reqwest::blocking::get(format!("https://pokeapi.co/api/v2/pokemon/{}", &no))?;
    Ok(response.json()?)
}

pub fn fetch(url: &str) -> Result<Bytes, Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?;
    Ok(response.bytes()?)
}

impl PokemonResult {
    pub fn fetch_sprite(&self) -> Result<Bytes, Box<dyn Error>> {
        fetch(&self.sprites.front)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn print_found_in_wild(&self) {
        println!("encountered a wild {} {}!", self.id, self.name);
    }
}
