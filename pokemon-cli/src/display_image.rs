use hyper::body::Bytes;
use image::EncodableLayout;
use show_image::{create_window, ImageInfo, ImageView};

pub fn display_image(window_name: &str, image: Bytes) -> Result<(), Box<dyn std::error::Error>> {
    let image = image::load_from_memory(&image)?.into_rgb8();
    let (width, height) = image.dimensions();
    let image_view = ImageView::new(ImageInfo::rgb8(width, height), &image.as_bytes());

    // Create a window with default options and display the image.
    let window = create_window(window_name, Default::default())?;
    window.set_image("image-001", image_view)?;

    Ok(window.wait_until_destroyed()?)
}
