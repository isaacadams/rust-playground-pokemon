extern crate rustc_serialize;
use std::{io::{Error}};
use image::EncodableLayout;
use rand::Rng;
use show_image::{ImageView, ImageInfo, create_window};
use reqwest;
use rustc_serialize::{json::Json};
use hyper::{Uri};

#[show_image::main]
fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap();

    match command.as_str() {
        "wild" => search_for_wild_pokemon(),
        &_ => panic!(),
    };
}

fn display_pokemon_with_url(pokemon_name: &String, no: &String) -> Result<(), Box<dyn std::error::Error>> {
    //println!("found a wild {} {}!!", &pokemon_name, &sprite_url);
    //let url: Uri = sprite_url.parse().unwrap();
    //println!("{} {}", &url.host().unwrap(), &url.path());
    //println!("{}", &sprite_url);

    let response = reqwest::blocking::get(format!("https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/{}.png", &no))?;
    let img_bytes = &response.bytes()?;
    let image = image::load_from_memory(&img_bytes)?.into_bgr8();
    let (width, height) = image.dimensions();
    let image_view = ImageView::new(ImageInfo::bgr8(width, height), &image.as_bytes());

    // Create a window with default options and display the image.
    let window = create_window(pokemon_name, Default::default())?;
    window.set_image("image-001", image_view)?;

    Ok(window.wait_until_destroyed()?)
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
    let index = rand::thread_rng().gen_range(0,800);
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

struct Pokemon {
    number: String,
    name: String,
}

impl Pokemon {
    fn get_pokedex_no(&self) -> i32 {
        let no = &self.number.parse::<i32>().unwrap();
        return *no;
    }

    fn display(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sprite_url = format!("https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/{}.png", &self.get_pokedex_no());
        display_pokemon_with_url(&self.name, &sprite_url)?;
        Ok(())
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

    fn save(&self) -> Result<(), Error> {
        let pokemon = &self.pokemon;
        let lines: Vec<String> = pokemon.into_iter().map(|pokemon|{ format!("{} {}", pokemon.number, pokemon.name) }).collect();
        let content = lines.join("\n");

        std::fs::write("pokedex.db", content)?;
        Ok(())
    }
}