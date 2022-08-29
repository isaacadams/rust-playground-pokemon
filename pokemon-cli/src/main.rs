mod display_image;

#[show_image::main]
fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap();

    let result = match command.as_str() {
        "wild" => search_wild(),
        &_ => panic!(),
    };

    match result {
        Err(e) => panic!("{}", e),
        _ => (),
    }
}

fn search_wild() -> Result<(), Box<dyn std::error::Error>> {
    let pokemon = pokemon_api::search_for_wild_pokemon();
    display_image::display_image(pokemon.name(), pokemon.fetch_sprite()?.to_image()?)
}
