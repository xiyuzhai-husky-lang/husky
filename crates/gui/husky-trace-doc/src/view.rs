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
    figure: Option<&'a TraceProtocol::Figure>,
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

    fn render_central_region(&mut self, ui: &mut egui::Ui) {
        // SidePanel::right(ui.auto_id_with("central_right"))
        //     .frame(Frame::none().inner_margin(0.0))
        //     .exact_width(ui.available_width() / 2.0)
        //     .resizable(false)
        //     // .exact_width(ui.available_width() / 2.0)
        //     .show_inside(ui, |ui| );
        let forest_desired_size = vec2(ui.available_width() * 0.5, ui.available_height() - 40.0);
        ui.horizontal(|ui| {
            ui.allocate_ui(forest_desired_size, |ui| self.render_forest(ui));
            ui.separator();
            self.render_central_right_region(ui);
        });
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

    TraceProtocol::Figure: ui::visual_widget::VisualWidget<egui::Ui>,
    Settings: HasTraceDocSettings,
{
    pub(crate) fn render(mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            self.render_central_region(ui);
            ui.separator();
            self.render_pedestal(ui);
        });
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
