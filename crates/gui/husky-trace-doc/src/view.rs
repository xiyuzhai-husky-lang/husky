mod components;
mod layout;
pub(crate) mod settings;

use crate::*;
use egui::{
    vec2, Align, Button, CentralPanel, Color32, FontFamily, Frame, InnerResponse, Label, LayerId,
    Layout, Margin, RichText, Sense, SidePanel, TextStyle, TextureId, TopBottomPanel, Vec2, Widget,
};
use husky_task_interface::val_control_flow::ValControlFlow;
use husky_trace_protocol::{
    figure::IsFigure,
    id::{TraceId, TraceKind},
    protocol::IsTraceProtocol,
    stalk::{JsonValue, TraceStalk},
    synchrotron::{TraceSynchrotron, TraceSynchrotronEntry},
    view::{action::TraceViewActionBuffer, TraceViewLineData, TraceViewTokenData},
};
use husky_value_protocol::presentation::ValuePresentation;
use std::path::Path;
use ui::ui::egui::UiCache;

pub(crate) struct TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,

    TraceProtocol::Figure: ui::visual_widget::VisualWidget<egui::Ui>,
    Settings: HasTraceDocSettings,
{
    current_dir: &'a Path,
    trace_synchrotron: &'a TraceSynchrotron<TraceProtocol>,
    figure: &'a TraceProtocol::Figure,
    action_buffer: &'a mut TraceViewActionBuffer<TraceProtocol>,
    settings: &'a mut Settings,
    ui_cache: &'a mut UiCache,
    glyph_width: f32,
}

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,

    TraceProtocol::Figure: ui::visual_widget::VisualWidget<egui::Ui>,
    Settings: HasTraceDocSettings,
{
    pub(crate) fn new(
        current_dir: &'a Path,
        trace_synchrotron: &'a TraceSynchrotron<TraceProtocol>,
        action_buffer: &'a mut TraceViewActionBuffer<TraceProtocol>,
        settings: &'a mut Settings,
        ui_cache: &'a mut UiCache,
        ui: &mut egui::Ui,
    ) -> Self {
        let glyph_width =
            ui.fonts(|f| f.glyph_width(&TextStyle::Monospace.resolve(ui.style()), ' '));
        Self {
            current_dir,
            trace_synchrotron,
            figure: trace_synchrotron.figure(),
            action_buffer,
            settings,
            glyph_width,
            ui_cache,
        }
    }

    fn add_action(&mut self, action: TraceViewAction<TraceProtocol>)
    where
        TraceProtocol: IsTraceProtocol,
    {
        self.action_buffer.push(action)
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
