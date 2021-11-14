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

    let available_pokemon = pokemon_in_wild();
    let index = rand::thread_rng().gen_range(0,available_pokemon.len());
    let encountered_pokemon = available_pokemon[index];

    println!("found a wild {}!!", encountered_pokemon);
}

fn pokemon_in_wild() -> [&'static str; 3] {
    return ["charmander", "squirtle", "caterpie"];
}
