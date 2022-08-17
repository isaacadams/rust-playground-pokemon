use reqwest;
use rustc_serialize::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::ops::Deref;

use crate::pokemon::Pokemon;

pub struct PokemonAPIData {
    pokemon: Pokemon,
    sprite: String,
}

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

impl PokemonAPIData {
    pub fn get_pokemon_data(no: u32) -> Result<PokemonAPIData, reqwest::Error> {
        let data: PokemonResult =
            reqwest::blocking::get(format!("https://pokeapi.co/api/v2/pokemon/{}", &no))?.json()?;

        Ok(PokemonAPIData {
            pokemon: Pokemon::new(&data.name, data.id),
            sprite: data.sprites.front,
        })
    }
}

impl Deref for PokemonAPIData {
    type Target = Pokemon;

    fn deref(&self) -> &Self::Target {
        &self.pokemon
    }
}
