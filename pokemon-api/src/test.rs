use crate::{api, search_for_wild_pokemon, search_pokemon_by_name};
use image::EncodableLayout;

#[test]
fn rgba8_valid() {
    let p = search_for_wild_pokemon();

    let image = p.fetch_sprite().unwrap().to_image().unwrap();
    let (rgba8, dimensions) = image.to_rgba8();

    test(dimensions, rgba8.as_bytes());
}

#[test]
fn rgba8_flat_samples_valid() {
    let p = search_for_wild_pokemon();

    let image = p.fetch_sprite().unwrap().to_image().unwrap();
    let (rgba8, dimensions) = image.to_rgba8();

    test(dimensions, rgba8.as_flat_samples().as_slice());
}

fn calculate_len(dimensions: [usize; 2]) -> usize {
    dimensions[0] * dimensions[1] * 4
}

fn test(dimensions: [usize; 2], data: &[u8]) {
    let len = calculate_len(dimensions);
    println!("{}", len);
    assert_eq!(len, data.len())
}

#[test]
fn get_all_pokemon() {
    let root = api::get_all_pokemon().unwrap();
    assert!(root.count > 1000);
    assert_eq!(root.results[0].name, "bulbasaur");
}

#[test]
fn search_by_name() {
    let matches = search_pokemon_by_name("pid").unwrap().iter().fold(String::new(), |acc, p| {
        format!("{}, {}", acc, p.name)
    });

    assert!(matches.contains("pidgey") && matches.contains("pidgeot") && matches.contains("rapidash"));
}