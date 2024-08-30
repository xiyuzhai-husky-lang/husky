use super::*;
use ::egui::{Color32, Frame};
use husky_trace_protocol::{caryatid::CaryatidUi, synchrotron::TraceSynchrotron};
use ui::ui::IsUi;

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
            let (bg, text) = match windlass {
                Windlass::Specific(_) => (Color32::DARK_RED, "S"),
                Windlass::Generic { .. } => (Color32::DARK_GREEN, "G"),
            };
            Frame::none().inner_margin(2.0).fill(bg).show(ui, |ui| {
                ui.horizontal_centered(|ui| ui.non_selectable_label(text))
            });
            Frame::none()
                .inner_margin(2.0)
                .fill(Color32::DARK_GRAY)
                .show(ui, |ui| {
                    ui.horizontal_centered(|ui| {
                        ui.non_selectable_label(
                            trace_synchrotron
                                .item_path_presentation(item_path_id_interface)
                                .ident(),
                        )
                    })
                });
            Frame::none()
                .inner_margin(2.0)
                .fill(Color32::GRAY)
                .show(ui, |ui| {
                    ui.horizontal_centered(|ui| match windlass {
                        Windlass::Specific(var_id)
                        | Windlass::Generic {
                            base: Some(var_id), ..
                        } => {
                            ui.non_selectable_label(
                                trace_synchrotron
                                    .var_id_presentation(item_path_id_interface, var_id)
                                    .data(),
                            );
                        }
                        Windlass::Generic { base: None, limit } => {
                            ui.non_selectable_label("--");
                        }
                    })
                });
        }
    }
}
