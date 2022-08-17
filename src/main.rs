extern crate rustc_serialize;
use rand::Rng;
use std::io::Error;

mod api;
mod pokemon;
mod window;

use pokemon::Pokemon;
use window::display_image;

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

    let data = api::get_pokemon_data(pick_random_pokemon()).unwrap();
    let sprite_bytes = data.fetch_sprite().unwrap();
    //println!("found a wild {}!!", data.deref());
    display_image(data.name(), sprite_bytes).unwrap();
}

fn pick_random_pokemon() -> u32 {
    rand::thread_rng().gen_range(0..800)
}

struct Pokedex {
    pokemon: Vec<Pokemon>,
}

impl Pokedex {
    fn new() -> Result<Pokedex, Error> {
        let contents = std::fs::read_to_string("pokedex.db")?;
        let mut pokemon = Vec::new();

        for line in contents.lines() {
            let chunks: Vec<&str> = line.split(' ').collect();

            if chunks.len() != 2 {
                todo!("either the delimiter is wrong or there is corrupted data");
            }

            pokemon.push(Pokemon::from_strs(chunks[0], chunks[1]));
        }

        Ok(Pokedex { pokemon })
    }

    fn pick_random_pokemon(&self) -> &Pokemon {
        let available_pokemon = &self.pokemon;
        let index = rand::thread_rng().gen_range(0..available_pokemon.len());
        let encountered_pokemon = available_pokemon.get(index).unwrap();

        return encountered_pokemon;
    }

    fn save(&self) -> Result<(), Error> {
        let lines: Vec<String> = self
            .pokemon
            .iter()
            .map(|pokemon| pokemon.to_string())
            .collect();
        let content = lines.join("\n");

        std::fs::write("pokedex.db", content)?;
        Ok(())
    }
}
