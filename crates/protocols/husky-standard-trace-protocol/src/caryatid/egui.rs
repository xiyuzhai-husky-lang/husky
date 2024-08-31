use super::*;
use ::egui::{Color32, Frame, Label, Margin, Sense, Widget};
use husky_trace_protocol::{caryatid::CaryatidUi, synchrotron::TraceSynchrotron};
use ui::ui::IsUi;

impl CaryatidUi<::egui::Ui> for StandardCaryatid {
    fn caryatid_ui<TraceProtocol>(
        &self,
        trace_synchrotron: &TraceSynchrotron<TraceProtocol>,
        ui: &mut ::egui::Ui,
        ui_buffer: &mut Self::UiBuffer,
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
            let inner_margin = Margin {
                left: 3.0,
                right: 3.0,
                top: 2.0,
                bottom: 2.0,
            };
            Frame::none()
                .inner_margin(inner_margin)
                .fill(bg)
                .show(ui, |ui| ui.non_selectable_label(text));
            Frame::none()
                .inner_margin(inner_margin)
                .fill(Color32::DARK_GRAY)
                .show(ui, |ui| {
                    ui.non_selectable_label(
                        trace_synchrotron
                            .item_path_presentation(item_path_id_interface)
                            .ident(),
                    )
                });
            Frame::none()
                .inner_margin(inner_margin)
                .fill(Color32::GRAY)
                .show(ui, |ui| {
                    let var_id = windlass.var_id();
                    if let Some(var_id_edit) = ui_buffer.var_id_edit(item_path_id_interface) {
                        ui.text_edit_singleline(var_id_edit);
                    } else {
                        let text = var_id
                            .map(|var_id| {
                                trace_synchrotron
                                    .var_id_presentation(item_path_id_interface, var_id)
                                    .data()
                            })
                            .unwrap_or("--");
                        let label = Label::new(text).sense(Sense::click()).selectable(false);
                        let label_response = &label.ui(ui);
                        if label_response.clicked() {
                            ui_buffer.show_var_id_edit(
                                item_path_id_interface,
                                var_id,
                                trace_synchrotron,
                            );
                        }
                    }
                });
        }
    }
}
