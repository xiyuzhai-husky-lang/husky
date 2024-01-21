mod amazon;
mod congo;
mod guinea;
mod taiga;

use super::*;
impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,

    TraceProtocol::Figure: ui::visual_widget::VisualWidget<egui::Ui>,
    Settings: HasTraceDocSettings,
{
    pub(in crate::view) fn render_forest(&mut self, ui: &mut egui::Ui) {
        // todo: use layout style to determine
        self.render_amazon_forest(ui)
    }
}
