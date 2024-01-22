// DevTools
// Chrome DevTools is a set of web developer tools built directly into the Google Chrome browser. DevTools can help you edit pages on-the-fly and diagnose problems quickly, which helps you build better websites, faster.
use super::*;

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    TraceProtocol::Figure: ui::visual_widget::VisualWidget<egui::Ui>,
    Settings: HasTraceDocSettings,
{
    pub(in crate::view) fn render_devtools(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.label(RichText::new("Devtools").color(Color32::WHITE));
            ui.separator();
            // for debug
            ui.label(
                RichText::new(format!(
                    r#"followed trace id = {:?}
pointer interact position = {:?}
accompanying trace ids = {:?}"#,
                    self.trace_synchrotron.followed_trace_id(),
                    ui.input(|input| input.pointer.interact_pos()),
                    &**self.trace_synchrotron.accompanying_trace_ids(),
                ))
                .color(Color32::GRAY),
            );
        });
    }
}
