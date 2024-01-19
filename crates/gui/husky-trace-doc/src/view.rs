mod devtools;
mod figure;
mod forest;
mod pedestal;

use crate::*;
use egui::{
    vec2, Align, Button, CentralPanel, Color32, FontFamily, Frame, InnerResponse, Label, LayerId,
    Layout, Margin, RichText, Sense, SidePanel, TextStyle, TopBottomPanel, Vec2, Widget,
};
use husky_task_interface::val_control_flow::ValControlFlow;
use husky_trace_protocol::{
    id::{TraceId, TraceKind},
    protocol::IsTraceProtocol,
    stalk::{JsonValue, TraceStalk},
    synchrotron::{TraceSynchrotron, TraceSynchrotronEntry},
    view::{action::TraceViewActionBuffer, TraceViewLineData, TraceViewTokenData},
};
use husky_value_protocol::presentation::ValuePresentation;

pub(crate) struct TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    Settings: HasTraceViewDocSettings,
{
    trace_synchrotron: &'a TraceSynchrotron<TraceProtocol>,
    action_buffer: &'a mut TraceViewActionBuffer<TraceProtocol>,
    settings: &'a mut Settings,
    glyph_width: f32,
}

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    Settings: HasTraceViewDocSettings,
{
    pub(crate) fn new(
        trace_synchrotron: &'a TraceSynchrotron<TraceProtocol>,
        action_buffer: &'a mut TraceViewActionBuffer<TraceProtocol>,
        ui: &mut egui::Ui,
        settings: &'a mut Settings,
    ) -> Self {
        let glyph_width =
            ui.fonts(|f| f.glyph_width(&TextStyle::Monospace.resolve(ui.style()), ' '));
        Self {
            trace_synchrotron,
            action_buffer,
            settings,
            glyph_width,
        }
    }

    fn add_action(&mut self, action: TraceViewAction<TraceProtocol>)
    where
        TraceProtocol: IsTraceProtocol,
    {
        self.action_buffer.push(action)
    }

    fn render_central_region(&mut self, ui: &mut egui::Ui) {
        SidePanel::right(ui.auto_id_with("central_right"))
            .frame(Frame::none().inner_margin(0.0))
            .exact_width(ui.available_width() / 2.0)
            .resizable(false)
            // .exact_width(ui.available_width() / 2.0)
            .show_inside(ui, |ui| self.render_central_right_region(ui));
        CentralPanel::default()
            .frame(Frame::none().inner_margin(0.0))
            .show_inside(ui, |ui| self.render_forest(ui));
    }

    fn render_central_right_region(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            self.render_figure(ui);
            ui.separator();
            self.render_devtools(ui);
        });
    }
}

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    Settings: HasTraceViewDocSettings,
{
    pub(crate) fn render(mut self, ui: &mut egui::Ui) {
        self.render_pedestal(ui);
        self.render_central_region(ui)
    }
}

fn horizontal_split(
    left: impl FnOnce(&mut egui::Ui),
    right: impl FnOnce(&mut egui::Ui),
    ui: &mut egui::Ui,
) {
    let desired_size = Vec2::new(ui.available_width() / 2.0, ui.available_height());
    ui.allocate_ui_with_layout(desired_size, Layout::left_to_right(Align::Center), |ui| {
        left(ui);
        right(ui);
    });
}
