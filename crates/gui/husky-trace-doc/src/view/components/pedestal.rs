use super::*;
use egui::Frame;
use husky_trace_protocol::pedestal::PedestalUi;

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    TraceProtocol::Pedestal: PedestalUi<Ui>,
    Settings: HasTraceDocSettings,
{
    pub(in crate::view) fn render_pedestal(&mut self, ui: &mut egui::Ui) {
        self.trace_synchrotron.pedestal().pedestal_ui(
            ui,
            self.pedestal_ui_buffer,
            self.action_buffer,
        )
    }
}
