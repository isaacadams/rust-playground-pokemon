use std::{io::Error};
use rand::Rng;

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
    let dex = Pokedex::new().unwrap();

    let available_pokemon = dex.pokemon;
    let index = rand::thread_rng().gen_range(0,available_pokemon.len());
    let encountered_pokemon = available_pokemon.get(index).unwrap();

    println!("found a wild {} {}!!", encountered_pokemon.number, encountered_pokemon.name);
}

struct Pokedex {
    pokemon: Vec<Pokemon>,
}

struct Pokemon {
    number: String,
    name: String,
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

            let number = chunks[0].to_owned();
            let name = chunks[1].to_owned();

            pokemon.push(Pokemon {
                number,
                name
            });
        }

        Ok(Pokedex {
            pokemon,
        })
    }
}