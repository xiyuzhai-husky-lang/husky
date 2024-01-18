mod figure;
mod forest;
mod pedestal;

use crate::*;
use egui::{
    Align, Button, Color32, FontFamily, InnerResponse, Label, LayerId, Layout, Margin, RichText,
    Sense, TextStyle, Vec2, Widget,
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
}

impl<'a, TraceProtocol, Settings> egui::Widget for TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    Settings: HasTraceViewDocSettings,
{
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
            self.render_pedestal(ui);
            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                let desired_size = Vec2::new(ui.available_width() / 2.0, ui.available_height());
                ui.horizontal(|ui| {
                    ui.allocate_ui(desired_size, |ui| {
                        self.render_forest(ui);
                        ui.allocate_space(ui.available_size());
                    });
                    self.render_figure(ui);
                })
            })
        })
        .response
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
