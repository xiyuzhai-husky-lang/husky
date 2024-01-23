use super::*;

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,

    TraceProtocol::Figure: FigureUi<egui::Ui>,
    Settings: HasTraceDocSettings,
{
    pub(super) fn render_congo_forest(&mut self, ui: &mut egui::Ui) {
        todo!()
    }
}
