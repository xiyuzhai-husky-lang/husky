use super::*;
use husky_trace_protocol::{caryatid::CaryatidUi, synchrotron::TraceSynchrotron};

impl CaryatidUi<::egui::Ui> for StandardCaryatid {
    fn caryatid_ui<TraceProtocol>(
        &self,
        trace_synchrotron: &TraceSynchrotron<TraceProtocol>,
        ui: &mut ::egui::Ui,
        caryatid_buffer: &mut Self::UiBuffer,
        action_buffer: &mut husky_trace_protocol::view::action::TraceViewActionBuffer<
            TraceProtocol,
        >,
    ) where
        TraceProtocol: IsTraceProtocol<Pedestal = Self::Pedestal, Caryatid = Self>,
    {
        for &(item_path_id_interface, windlass) in &self.windlasses {
            match windlass {
                Windlass::Specific(_) => ui.label("S"),
                Windlass::Generic { .. } => ui.label("G"),
            };
            ui.label(
                trace_synchrotron
                    .item_path_presentation(item_path_id_interface)
                    .ident(),
            );
            match windlass {
                Windlass::Specific(var_id)
                | Windlass::Generic {
                    base: Some(var_id), ..
                } => {
                    ui.label(
                        trace_synchrotron
                            .var_id_presentation(item_path_id_interface, var_id)
                            .data(),
                    );
                }
                Windlass::Generic { base: None, limit } => {
                    ui.label("--");
                }
            }
        }
    }
}
