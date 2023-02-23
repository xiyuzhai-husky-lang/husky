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
        Box::new(|_cc| Box::new(HuskyNotebook::default())),
    )
}

struct HuskyNotebook {
    name: String,
    age: u32,
}

impl Default for HuskyNotebook {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: Default::default(),
        }
    }
}

impl eframe::App for HuskyNotebook {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Husky Notebook");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
            ui.label(format!(
                "Formula: {}xdx",
                husky_unicode_symbols::opr::OPR_INTEGRAL
            ))
        });
    }
}
