use crate::*;
use ui::IsUiComponent;

pub fn run_standalone_ui_component<UiComponent, UiComponentConfig>(
    component: UiComponent,
    config: UiComponentConfig,
) -> Result<(), eframe::Error>
where
    UiComponent: IsUiComponent<egui::Ui, UiComponentConfig> + 'static,
    UiComponentConfig: 'static,
{
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Standalone Ui Component",
        options,
        Box::new(|_cc| Box::new(StandaloneUiApp { component, config })),
    )
}

struct StandaloneUiApp<UiComponent, UiComponentConfig> {
    component: UiComponent,
    config: UiComponentConfig,
}

impl<UiComponent, UiComponentConfig> eframe::App for StandaloneUiApp<UiComponent, UiComponentConfig>
where
    UiComponent: IsUiComponent<egui::Ui, UiComponentConfig>,
{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| self.component.render(ui, &self.config));
    }
}
