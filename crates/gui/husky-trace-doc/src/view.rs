mod components;
mod layout;
pub(crate) mod settings;

use crate::*;
use egui::{
    vec2, Color32, FontFamily, Frame, Label, Margin, RichText, Sense, SidePanel, TextStyle,
    TopBottomPanel, Ui, Vec2, Widget,
};
use husky_devsoul_interface::{ki_control_flow::KiControlFlow, pedestal::IsPedestal};
use husky_trace_protocol::{
    figure::{FigureUi, FigureUiCache},
    id::{TraceId, TraceKind},
    protocol::IsTraceProtocol,
    stalk::TraceStalk,
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
    caryatid_ui_buffer: &'a mut <TraceProtocol::Caryatid as IsCaryatid>::UiBuffer,
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
        pedestal_ui_buffer: &'a mut <TraceProtocol::Caryatid as IsCaryatid>::UiBuffer,
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
            caryatid_ui_buffer: pedestal_ui_buffer,
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
