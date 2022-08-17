use hyper::body::Bytes;
use std::{error::Error, fmt::Display};

pub struct Pokemon {
    number: u32,
    name: String,
}

impl Pokemon {
    pub fn from_strs(name: &str, no: &str) -> Self {
        Pokemon {
            name: name.to_string(),
            number: no.parse::<u32>().unwrap(),
        }
    }

    pub fn new(name: &str, number: u32) -> Self {
        Pokemon {
            name: name.to_string(),
            number,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fetch_sprite(&self) -> Result<Bytes, Box<dyn Error>> {
        let response = reqwest::blocking::get(format!(
            "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/{}.png",
            self.number
        ))?;

        Ok(response.bytes()?)
    }
}

impl Display for Pokemon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}", self.name, self.number))
    }
}
