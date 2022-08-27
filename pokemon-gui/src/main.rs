#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::App;
use egui::{ColorImage, RichText, TextureHandle};
use image::EncodableLayout;

// When compiling natively:
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    //tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some([1280.0, 1024.0].into()),
        #[cfg(feature = "wgpu")]
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };
    eframe::run_native(
        "pokemon app",
        options,
        Box::new(|cc| {
            Box::new(MyApp {
                caught_pokemon: String::from(""),
                pokemon_sprite: None,
            })
        }),
    );
}

struct MyApp {
    caught_pokemon: String,
    pokemon_sprite: Option<TextureHandle>,
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.label(create_text("Search for Wild Pokemon!"));

            if ui.button(create_text("search")).clicked() {
                // take some action here
                let pokemon = pokemon_api::search_for_wild_pokemon();
                self.caught_pokemon = pokemon.name().to_string();
                let image = pokemon.fetch_sprite().unwrap();
                self.pokemon_sprite = Some(ctx.load_texture(
                    "pokemon",
                    load_image_from_memory(image.as_bytes().as_ref()).unwrap(),
                )); // ;
            }

            ui.label(create_text(&self.caught_pokemon));
            if let Some(i) = &self.pokemon_sprite {
                ui.image(i, i.size_vec2());
            }
        });
    }
}

fn create_text(text: &str) -> RichText {
    RichText::new(text).size(18.0)
}

fn load_image_from_memory(image_data: &[u8]) -> Result<ColorImage, image::ImageError> {
    let image = image::load_from_memory(image_data)?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()))
}
