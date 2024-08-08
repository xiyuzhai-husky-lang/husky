use super::*;
use husky_trace_protocol::caryatid::CaryatidUi;

impl CaryatidUi<::egui::Ui> for StandardCaryatid {
    fn caryatid_ui<TraceProtocol>(
        &self,
        ui: &mut ::egui::Ui,
        caryatid_buffer: &mut Self::UiBuffer,
        action_buffer: &mut husky_trace_protocol::view::action::TraceViewActionBuffer<
            TraceProtocol,
        >,
    ) where
        TraceProtocol: IsTraceProtocol<Caryatid = Self>,
    {
        ui.label("todo: caryatid ui");
    }
}
