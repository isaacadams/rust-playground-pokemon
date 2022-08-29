use image::EncodableLayout;
use pokemon_api::PokemonImage;
use show_image::{create_window, ImageInfo, ImageView};

pub fn display_image(
    window_name: &str,
    image: PokemonImage,
) -> Result<(), Box<dyn std::error::Error>> {
    let (rgba, dimensions) = image.to_rgba8();

    let image_view = ImageView::new(
        ImageInfo::rgba8(dimensions[0] as u32, dimensions[1] as u32),
        &rgba.as_bytes(),
    );

    // Create a window with default options and display the image.
    let window = create_window(window_name, Default::default())?;
    window.set_image("image-001", image_view)?;

    Ok(window.wait_until_destroyed()?)
}
