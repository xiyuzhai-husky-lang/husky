mod control;
mod explorer;
mod figure;
mod menu;

use self::control::*;
use self::explorer::*;
use self::figure::*;
use self::menu::*;

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Template Matching Viewer",
        options,
        Box::new(|_cc| Box::new(App::default())),
    )
}

struct App {
    explorer: Explorer,
    figure: Figure,
    control: Control,
}

impl Default for App {
    fn default() -> Self {
        Self {
            explorer: Default::default(),
            figure: Default::default(),
            control: Default::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render_menu_panel(ctx);
        self.render_explorer_panel(ctx);
        self.render_control_panel(ctx);
        self.render_figure_panel(ctx);
    }
}
