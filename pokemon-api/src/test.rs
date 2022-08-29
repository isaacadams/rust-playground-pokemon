use crate::search_for_wild_pokemon;
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
