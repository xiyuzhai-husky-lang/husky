mod components;
mod config;
mod panels;

use self::config::*;
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
    doctree: egui_dock::Tree<String>,
}

impl Default for HuskyNotebookApp {
    fn default() -> Self {
        let tab1 = "tab1".to_string();
        let tab2 = "tab2".to_string();

        let mut doctree = egui_dock::Tree::new(vec![tab1]);
        doctree.split_left(egui_dock::NodeIndex::root(), 0.20, vec![tab2]);
        Self {
            config: HuskyNotebookConfig::default(),
            doctree,
        }
    }
}

impl eframe::App for HuskyNotebookApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render_panels(ctx)
        // egui::CentralPanel::default().show(ctx, |ui|);
    }
}
