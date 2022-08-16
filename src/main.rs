extern crate rustc_serialize;
use std::{io::{Error}};
use rand::Rng;
use reqwest;
use rustc_serialize::{json::Json};

mod pokemon;

use pokemon::{Pokemon, display_pokemon_with_url};

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

    //let dex = Pokedex::new().unwrap();
    //let encountered_pokemon = dex.pick_random_pokemon();
    //let data = get_pokemon_data(encountered_pokemon.get_pokedex_no()).unwrap();

    let data = get_pokemon_data(pick_random_pokemon()).unwrap();
    println!("found a wild {} {}!!", data.name, data.sprite);
    display_pokemon_with_url(&data.name, &data.no).unwrap();
}

fn get_pokemon_data(no: i32) -> Result<PokemonAPIData, reqwest::Error> {
    let data = reqwest::blocking::get(format!("https://pokeapi.co/api/v2/pokemon/{}", &no))?.text()?;
    let json = Json::from_str(&data).unwrap();
    
    Ok(PokemonAPIData {
        name: json.find_path(&["name"]).unwrap().to_string().to_owned(),
        no: json.find_path(&["id"]).unwrap().to_string().to_owned(),
        sprite: json.find_path(&["sprites", "front_default"]).unwrap().to_string().to_owned()
    })
}

fn pick_random_pokemon() -> i32 {
    let index = rand::thread_rng().gen_range(0..800);
    return index;
}

struct PokemonAPIData {
    name: String,
    no: String,
    sprite: String,
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

            pokemon.push(Pokemon::new(chunks[0].to_owned(), chunks[1].to_owned()));
        }

        Ok(Pokedex {
            pokemon,
        })
    }

    fn pick_random_pokemon(&self) -> &Pokemon {
        let available_pokemon = &self.pokemon;
        let index = rand::thread_rng().gen_range(0..available_pokemon.len());
        let encountered_pokemon = available_pokemon.get(index).unwrap();

        return encountered_pokemon;
    }

    fn save(&self) -> Result<(), Error> {
        let lines: Vec<String> = self.pokemon.iter().map(|pokemon|{ pokemon.to_string() }).collect();
        let content = lines.join("\n");

        std::fs::write("pokedex.db", content)?;
        Ok(())
    }
}