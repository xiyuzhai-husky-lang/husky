mod app;

use self::app::TexEguiApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        // initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Husky Mnist Game",
        options,
        Box::new(|_cc| Box::new(TexEguiApp::default())),
    )
}
