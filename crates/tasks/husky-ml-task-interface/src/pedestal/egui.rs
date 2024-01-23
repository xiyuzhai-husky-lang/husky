use super::*;
use ::egui::{vec2, Button, Color32, Frame, Label, RichText, Ui, Widget};
use husky_trace_protocol::{pedestal::PedestalUi, view::action::TraceViewAction};

impl PedestalUi<Ui> for MlPedestal {
    fn pedestal_ui<TraceProtocol>(
        self,
        ui: &mut Ui,
        pedestal_ui_buffer: &mut Self::UiBuffer,
        action_buffer: &mut husky_trace_protocol::view::action::TraceViewActionBuffer<
            TraceProtocol,
        >,
    ) where
        TraceProtocol: husky_trace_protocol::protocol::IsTraceProtocol<Pedestal = Self>,
    {
        // todo: style this
        let text = match self {
            MlPedestal::Specific(_) => "SPECIFIC",
            MlPedestal::Generic => "GENERIC",
        };
        ui.vertical(|ui| {
            ui.allocate_space(vec2(0.0, 2.0));
            ui.horizontal(|ui| {
                ui.style_mut().spacing.item_spacing.x = 5.0;
                Button::new(text)
                    .fill(Color32::from_rgb(128, 0, 128))
                    .stroke((0.0, Color32::WHITE))
                    .ui(ui);
                ui.label("input id = ");
                if ui
                    .text_edit_singleline(&mut pedestal_ui_buffer.input_id_to_be)
                    .lost_focus()
                {
                    match pedestal_ui_buffer.input_id_to_be.parse::<usize>() {
                        Ok(index) => action_buffer.push(TraceViewAction::SetPedestal {
                            pedestal: MlPedestal::Specific(InputId::from_index(index)),
                        }),
                        Err(e) => pedestal_ui_buffer.error = Some(e.to_string()),
                    }
                }
                if let Some(ref e) = pedestal_ui_buffer.error {
                    ui.label(RichText::new(e).color(Color32::RED));
                }
            })
        });
    }
}
