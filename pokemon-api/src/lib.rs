extern crate rustc_serialize;

use rand::Rng;

#[cfg(test)]
mod test;

mod api;
#[allow(dead_code)]
mod pokedex;
mod sprite;

use api::PokemonResult;
pub use sprite::PokemonImage;

pub fn search_for_wild_pokemon() -> PokemonResult {
    println!("searching for wild pokemon");

    let random_pokemon_no = pick_random_pokemon();
    let data = api::get_pokemon_data(random_pokemon_no).unwrap();
    data.print_found_in_wild();
    data
}

fn pick_random_pokemon() -> u32 {
    rand::thread_rng().gen_range(1..905)
}
