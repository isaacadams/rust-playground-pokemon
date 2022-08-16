use std::fmt::Display;

use image::EncodableLayout;
use show_image::{ImageView, ImageInfo, create_window};

pub struct Pokemon {
    number: String,
    name: String,
}

impl Pokemon {
    pub fn new(name: String, number: String) -> Self {
        Pokemon {
            name,
            number
        }
    }

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

impl Display for Pokemon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}", self.name, self.number))
    }
}

pub fn display_pokemon_with_url(pokemon_name: &String, no: &String) -> Result<(), Box<dyn std::error::Error>> {
    //println!("found a wild {} {}!!", &pokemon_name, &sprite_url);
    //let url: Uri = sprite_url.parse().unwrap();
    //println!("{} {}", &url.host().unwrap(), &url.path());
    //println!("{}", &sprite_url);

    let response = reqwest::blocking::get(format!("https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/{}.png", &no))?;
    let img_bytes = &response.bytes()?;
    //let image = image::load_from_memory(&img_bytes)?.into_bgr8();
    let image = image::load_from_memory(&img_bytes)?.into_rgb8();
    let (width, height) = image.dimensions();
    let image_view = ImageView::new(ImageInfo::bgr8(width, height), &image.as_bytes());

    // Create a window with default options and display the image.
    let window = create_window(pokemon_name, Default::default())?;
    window.set_image("image-001", image_view)?;

    Ok(window.wait_until_destroyed()?)
}