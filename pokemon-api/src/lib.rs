extern crate rustc_serialize;

use rand::Rng;

#[cfg(test)]
mod test;

mod api;
mod api_models;
#[allow(dead_code)]
mod pokedex;
mod sprite;

use api_models::PokemonResult;
pub use sprite::PokemonImage;

/// 902 throws an error because it does not have the sprite url that most pokemon have
pub fn search_for_wild_pokemon() -> PokemonResult {
    println!("searching for wild pokemon");

    let random_pokemon_no = pick_random_pokemon();
    println!("searching for {}", random_pokemon_no);
    let data = api::get_pokemon_data(random_pokemon_no).unwrap();
    data.print_found_in_wild();
    data
}

pub fn search_pokemon_by_name(name: &str) -> Result<Vec<api_models::NamedAPIResource>, reqwest::Error> {
    let pokemon = api::get_all_pokemon()?;
    Ok(pokemon.results.into_iter().filter(|p| p.name.contains(name)).collect())
}

fn pick_random_pokemon() -> u32 {
    rand::thread_rng().gen_range(1..905)
}
