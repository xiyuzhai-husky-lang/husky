use super::*;

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    Settings: HasTraceViewDocSettings,
{
    pub(super) fn render_pedestal(&mut self, ui: &mut egui::Ui) {
        ui.label(format!(
            "pedestal = {:?}",
            self.trace_synchrotron.pedestal()
        ));
    }
}
