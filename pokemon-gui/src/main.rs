#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::App;
use egui::RichText;

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
            })
        }),
    );
}

struct MyApp {
    caught_pokemon: String,
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.label(create_text("Search for Wild Pokemon!"));

            if ui.button(create_text("search")).clicked() {
                // take some action here
                let pokemon = pokemon_api::search_for_wild_pokemon();
                self.caught_pokemon = pokemon.name().to_string();
            }

            ui.label(create_text(&self.caught_pokemon));
            //ui.image(texture_id, size)
        });
    }
}

fn create_text(text: &str) -> RichText {
    RichText::new(text).size(18.0)
}
