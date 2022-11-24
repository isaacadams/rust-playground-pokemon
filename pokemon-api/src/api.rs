use crate::{
    api_models::{NamedAPIResourceList, PokemonResult},
};

const BASE: &str = "https://pokeapi.co/api/v2/";
const POKEMON: &str = "pokemon";

pub fn get_pokemon_data(no: u32) -> Result<PokemonResult, reqwest::Error> {
    let response = reqwest::blocking::get(format!("{}/{}/{}", BASE, POKEMON, &no))?;
    response.json()
}

pub fn get_all_pokemon() -> Result<NamedAPIResourceList, reqwest::Error> {
    let response = reqwest::blocking::get(format!("{}/{}?limit=3000", BASE, POKEMON))?;
    response.json()
}
