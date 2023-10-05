mod action;
mod components;
mod doc;
mod layout;
mod settings;

use self::action::*;
use self::doc::{DocTab, Docs};
use self::settings::*;
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
        Box::new(|_cc| Box::new(NotebookApp::default())),
    )
}

struct NotebookApp {
    settings: NotebookSettings,
    dock_state: egui_dock::DockState<DocTab>,
    docs: Docs,
    action_buffer: NotebookActionBuffer,
}

impl Default for NotebookApp {
    fn default() -> Self {
        let action_buffer = Default::default();
        let mut dock_state = egui_dock::DockState::new(vec![]);
        let docs = Docs::default();
        let config = Default::default();
        let mut slf = Self {
            settings: config,
            dock_state,
            docs,
            action_buffer,
        };
        slf.add_default_docs();
        slf
    }
}

impl eframe::App for NotebookApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render_panels(ctx)
        // egui::CentralPanel::default().show(ctx, |ui|);
    }
}
