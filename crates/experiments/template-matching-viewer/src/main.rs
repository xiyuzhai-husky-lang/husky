fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Template Matching Viewer",
        options,
        Box::new(|_cc| Box::new(TemplateMatchingViewerApp::default())),
    )
}

struct TemplateMatchingViewerApp {
    t: usize,
}

impl Default for TemplateMatchingViewerApp {
    fn default() -> Self {
        Self {
            t: Default::default(),
        }
    }
}

impl eframe::App for TemplateMatchingViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // self.render_panels(ctx)
        // egui::CentralPanel::default().show(ctx, |ui|);
    }
}
