extern crate rustc_serialize;
use rand::Rng;

mod api;
mod pokedex;
mod window;

#[show_image::main]
fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap();

    match command.as_str() {
        "wild" => search_for_wild_pokemon(),
        &_ => panic!(),
    };
}

fn search_for_wild_pokemon() {
    println!("searching for wild pokemon");

    let random_pokemon_no = pick_random_pokemon();
    let data = api::get_pokemon_data(random_pokemon_no).unwrap();
    data.print_found_in_wild();
    let sprite_bytes = data.fetch_sprite().unwrap();
    window::display_image(data.name(), sprite_bytes).unwrap();
}

fn pick_random_pokemon() -> u32 {
    rand::thread_rng().gen_range(0..800)
}