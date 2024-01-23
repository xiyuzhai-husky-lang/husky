mod components;
mod layout;
pub(crate) mod settings;

use crate::*;
use egui::{
    vec2, Align, Button, CentralPanel, Color32, FontFamily, Frame, InnerResponse, Label, LayerId,
    Layout, Margin, RichText, Sense, SidePanel, TextStyle, TextureId, TopBottomPanel, Ui, Vec2,
    Widget,
};
use husky_task_interface::{pedestal::IsPedestal, val_control_flow::ValControlFlow};
use husky_trace_protocol::{
    figure::{FigureUi, FigureUiCache, IsFigure},
    id::{TraceId, TraceKind},
    pedestal::PedestalUi,
    protocol::IsTraceProtocol,
    stalk::{JsonValue, TraceStalk},
    synchrotron::{TraceSynchrotron, TraceSynchrotronEntry},
    view::{action::TraceViewActionBuffer, TraceViewLineData, TraceViewTokenData},
};
use husky_value_protocol::presentation::ValuePresentation;
use std::path::Path;

pub(crate) struct TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    Settings: HasTraceDocSettings,
{
    current_dir: &'a Path,
    trace_synchrotron: &'a TraceSynchrotron<TraceProtocol>,
    figure: &'a TraceProtocol::Figure,
    action_buffer: &'a mut TraceViewActionBuffer<TraceProtocol>,
    settings: &'a mut Settings,
    figure_ui_cache: &'a mut FigureUiCache<Ui>,
    pedestal_ui_buffer: &'a mut <TraceProtocol::Pedestal as IsPedestal>::UiBuffer,
    glyph_width: f32,
}

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    Settings: HasTraceDocSettings,
{
    pub(crate) fn new(
        current_dir: &'a Path,
        trace_synchrotron: &'a TraceSynchrotron<TraceProtocol>,
        action_buffer: &'a mut TraceViewActionBuffer<TraceProtocol>,
        settings: &'a mut Settings,
        figure_ui_cache: &'a mut FigureUiCache<Ui>,
        pedestal_ui_buffer: &'a mut <TraceProtocol::Pedestal as IsPedestal>::UiBuffer,
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
            pedestal_ui_buffer,
            figure_ui_cache,
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
