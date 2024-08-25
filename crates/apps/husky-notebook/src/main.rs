pub mod action;
mod components;
mod doc;
mod layout;
mod settings;

use std::sync::Arc;

use self::action::*;
use self::doc::{DocTab, Docs};
use self::settings::*;
use eframe::egui;
use ui::hotkey::egui::HotkeyBuffer;

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        // initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Husky Notebook",
        options,
        Box::new(|_cc| Ok(Box::new(NotebookApp::default()))),
    )
}

struct NotebookApp {
    dock_state: egui_dock::DockState<DocTab>,
    docs: Docs,
    settings: NotebookSettings,
    hotkey_buffer: HotkeyBuffer,
    action_buffer: NotebookActionBuffer,
    tokio_runtime: Arc<tokio::runtime::Runtime>,
    init_done: bool,
}

impl Default for NotebookApp {
    fn default() -> Self {
        let action_buffer = Default::default();
        let dock_state = egui_dock::DockState::new(vec![]);
        let docs = Docs::default();
        let settings = Default::default();
        Self {
            settings,
            dock_state,
            docs,
            hotkey_buffer: Default::default(),
            action_buffer,
            tokio_runtime: Arc::new(tokio::runtime::Runtime::new().unwrap()),
            init_done: false,
        }
    }
}

impl eframe::App for NotebookApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.init_done {
            self.init(ctx);
            self.init_done = true;
        }
        self.hotkey_buffer.start_frame(ctx);
        self.render_panels(ctx)
    }
}

impl NotebookApp {
    fn init(&mut self, ctx: &egui::Context) {
        self.add_default_docs(ctx);
    }
}
