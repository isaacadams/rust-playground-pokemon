use hyper::body::Bytes;
use reqwest;
use rustc_serialize::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::ops::Deref;

use crate::pokemon::Pokemon;

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
    /* #[serde(flatten)]
    extra: HashMap<String, Value>, */
}

pub fn get_pokemon_data(no: u32) -> Result<PokemonResult, reqwest::Error> {
    let response = reqwest::blocking::get(format!("https://pokeapi.co/api/v2/pokemon/{}", &no))?;
    //println!("{:#?}", response);
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
}

/* impl Deref for PokemonResult {
    type Target = Pokemon;

    fn deref(&self) -> &Self::Target {
        &self.pokemon
    }
} */
