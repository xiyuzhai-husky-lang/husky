use super::*;

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,

    TraceProtocol::Figure: ui::visual_widget::VisualWidget<egui::Ui>,
    Settings: HasTraceDocSettings,
{
    pub(in super::super) fn render_token(
        &mut self,
        token_data: &TraceViewTokenData,
        trace_id: TraceId,
        entry: &TraceSynchrotronEntry<TraceProtocol>,
        ui: &mut egui::Ui,
    ) {
        let token_foreground_colors = self
            .settings
            .code_editor_settings()
            .token_foreground_colors();
        let spaces_before = token_data.spaces_before();
        self.render_space_chars(spaces_before, ui);
        let mut label = Label::new(
            RichText::new(token_data.text())
                .family(FontFamily::Monospace)
                .color(token_foreground_colors[token_data.token_class()]),
        );
        if token_data.associated_trace_id().is_some() {
            label = label.sense(Sense::click());
        }
        let label_response = label.ui(ui);
        if let Some(associated_trace_id) = token_data.associated_trace_id() {
            if label_response.clicked() {
                self.add_action(TraceViewAction::ToggleAssociatedTrace {
                    trace_id,
                    associated_trace_id,
                })
            }
            if entry.associated_trace_ids().contains(&associated_trace_id) {
                label_response.highlight();
            } else if label_response.hovered() {
                label_response.highlight();
            }
        }
    }
}
