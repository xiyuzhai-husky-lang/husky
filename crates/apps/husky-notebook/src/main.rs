mod components;
mod config;
mod panels;

use self::config::*;
use components::doc::DocsDock;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Husky Notebook",
        options,
        Box::new(|_cc| Box::new(HuskyNotebookApp::default())),
    )
}

struct HuskyNotebookApp {
    config: HuskyNotebookConfig,
    docs_dock: DocsDock,
}

impl Default for HuskyNotebookApp {
    fn default() -> Self {
        let tab1 = "tab1".to_string();
        let tab2 = "tab2".to_string();

        let mut dock_state = egui_dock::DockState::new(vec![tab1, tab2]);
        Self {
            config: HuskyNotebookConfig::default(),
            docs_dock: DocsDock::default(),
        }
    }
}

impl eframe::App for HuskyNotebookApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render_panels(ctx)
        // egui::CentralPanel::default().show(ctx, |ui|);
    }
}
