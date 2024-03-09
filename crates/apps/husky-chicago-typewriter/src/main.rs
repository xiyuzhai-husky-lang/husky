mod action;
mod app;
mod db;
mod view;

use self::app::ChicagoTypewriterApp;

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        // initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Husky Chicago Typewriter",
        options,
        Box::new(|_cc| Box::new(ChicagoTypewriterApp::default())),
    )
}
