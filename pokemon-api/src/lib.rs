extern crate rustc_serialize;
use api::PokemonResult;
use rand::Rng;

mod api;
mod pokedex;

pub fn search_for_wild_pokemon() -> PokemonResult {
    println!("searching for wild pokemon");

    let random_pokemon_no = pick_random_pokemon();
    let data = api::get_pokemon_data(random_pokemon_no).unwrap();
    data.print_found_in_wild();
    data
    /* let sprite_bytes = data.fetch_sprite().unwrap();
    window::display_image(data.name(), sprite_bytes).unwrap(); */
}

fn pick_random_pokemon() -> u32 {
    rand::thread_rng().gen_range(1..905)
}
