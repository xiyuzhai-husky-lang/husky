use crate::*;
use ui::IsUiComponent;

pub fn run_standalone_ui_component<UiComponent, UiComponentConfig, UiActionBuffer>(
    component: UiComponent,
    config: UiComponentConfig,
    action_buffer: UiActionBuffer,
) -> Result<(), eframe::Error>
where
    UiComponent: IsUiComponent<egui::Ui, UiComponentConfig, UiActionBuffer> + 'static,
    UiComponentConfig: 'static,
    UiActionBuffer: 'static,
{
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Standalone Ui Component",
        options,
        Box::new(|_cc| {
            Box::new(StandaloneUiApp {
                component,
                settings: config,
                action_buffer,
            })
        }),
    )
}

struct StandaloneUiApp<UiComponent, UiComponentSettings, UiActionBuffer> {
    component: UiComponent,
    settings: UiComponentSettings,
    action_buffer: UiActionBuffer,
}

impl<UiComponent, UiComponentConfig, UiActionBuffer> eframe::App
    for StandaloneUiApp<UiComponent, UiComponentConfig, UiActionBuffer>
where
    UiComponent: IsUiComponent<egui::Ui, UiComponentConfig, UiActionBuffer>,
{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.component
                .update(ui, &mut self.settings, &mut self.action_buffer)
        });
    }
}
