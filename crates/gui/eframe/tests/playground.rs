struct MyApp;

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| ui.label("Normal text"));
    }
}

#[test]
fn run_me() {
    todo!()
}
