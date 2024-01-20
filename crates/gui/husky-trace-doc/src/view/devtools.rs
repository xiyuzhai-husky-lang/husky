// DevTools
// Chrome DevTools is a set of web developer tools built directly into the Google Chrome browser. DevTools can help you edit pages on-the-fly and diagnose problems quickly, which helps you build better websites, faster.
use super::*;

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    TraceProtocol::Figure: egui::Widget,
    Settings: HasTraceViewDocSettings,
{
    pub(super) fn render_devtools(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.label(RichText::new("Devtools").color(Color32::WHITE));
            ui.separator();
            // for debug
            ui.label(
                RichText::new(format!(
                    r#"self.trace_synchrotron.focused_trace_id() = {:?}
input.pointer.interact_pos() = {:?}
ui.available_size() = {:?}"#,
                    self.trace_synchrotron.followed_trace_id(),
                    ui.input(|input| input.pointer.interact_pos()),
                    ui.available_size()
                ))
                .color(Color32::GRAY),
            );
            ui.horizontal(|ui| {
                let available_size = ui.available_size();
                ui.label(format!("available size for control = {available_size:?}"))
            });
        });
    }
}
