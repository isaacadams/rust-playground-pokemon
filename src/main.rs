use std::{io::Error};
use image::EncodableLayout;
use rand::Rng;
use show_image::{ImageView, ImageInfo, create_window};
use reqwest;

#[show_image::main]
fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap();

    match command.as_str() {
        "wild" => search_for_wild_pokemon(),
        &_ => panic!(),
    };
}

fn display_pokemon(pokemon: &Pokemon) -> Result<(), Box<dyn std::error::Error>> {

    let response = reqwest::blocking::get(format!("https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/{}.png", &pokemon.get_pokedex_no()))?;
    let img_bytes = &response.bytes()?;
    let image = image::load_from_memory(&img_bytes)?.into_bgr8();
    let (width, height) = image.dimensions();
    let image_view = ImageView::new(ImageInfo::bgr8(width, height), &image.as_bytes());

    // Create a window with default options and display the image.
    let window = create_window(&pokemon.name, Default::default())?;
    window.set_image("image-001", image_view)?;

    Ok(window.wait_until_destroyed()?)
}

fn search_for_wild_pokemon() {
    println!("searching for wild pokemon");

    let dex = Pokedex::new().unwrap();
    let encountered_pokemon = dex.pick_random_pokemon();

    println!("found a wild {} {}!!", encountered_pokemon.number, encountered_pokemon.name);

    display_pokemon(&encountered_pokemon);
}

struct Pokedex {
    pokemon: Vec<Pokemon>,
}

struct Pokemon {
    number: String,
    name: String,
}

impl Pokemon {
    fn get_pokedex_no(&self) -> i32 {
        let no = &self.number.parse::<i32>().unwrap();
        return *no;
    }
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

            pokemon.push(Pokemon {
                number: chunks[0].to_owned(),
                name: chunks[1].to_owned(),
            });
        }

        Ok(Pokedex {
            pokemon,
        })
    }

    fn pick_random_pokemon(&self) -> &Pokemon {
        let available_pokemon = &self.pokemon;
        let index = rand::thread_rng().gen_range(0,available_pokemon.len());
        let encountered_pokemon = available_pokemon.get(index).unwrap();

        return encountered_pokemon;
    }
}