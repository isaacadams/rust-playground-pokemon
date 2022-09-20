use rand::Rng;
use std::fmt::Display;
use std::io::Error;

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
}

impl Display for Pokemon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}", self.name, self.number))
    }
}

pub struct Pokedex {
    pokemon: Vec<Pokemon>,
}

impl Pokedex {
    pub fn new() -> Result<Pokedex, Error> {
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

    pub fn pick_random_pokemon(&self) -> &Pokemon {
        let available_pokemon = &self.pokemon;
        let index = rand::thread_rng().gen_range(0..available_pokemon.len());
        available_pokemon.get(index).unwrap()
    }

    pub fn save(&self) -> Result<(), Error> {
        let lines: Vec<String> = self
            .pokemon
            .iter()
            .map(|pokemon| format!("{} {}", pokemon.number, pokemon.name))
            .collect();

        let content = lines.join("\n");

        std::fs::write("pokedex.db", content)?;
        Ok(())
    }
}
