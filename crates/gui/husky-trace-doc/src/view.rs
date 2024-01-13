use crate::*;
use egui::{
    Button, Color32, FontFamily, InnerResponse, Label, Margin, RichText, Sense, TextStyle, Vec2,
    Widget,
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
    trace_cache: &'a TraceSynchrotron<TraceProtocol>,
    action_buffer: &'a mut TraceViewActionBuffer<TraceProtocol>,
    settings: &'a mut Settings,
    // cached values
    glyph_width: f32,
    // trace_listing: Vec<TraceId>,
}

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    Settings: HasTraceViewDocSettings,
{
    pub(crate) fn new(
        trace_cache: &'a TraceSynchrotron<TraceProtocol>,
        action_buffer: &'a mut TraceViewActionBuffer<TraceProtocol>,
        ui: &mut egui::Ui,
        settings: &'a mut Settings,
    ) -> Self {
        let glyph_width =
            ui.fonts(|f| f.glyph_width(&TextStyle::Monospace.resolve(ui.style()), ' '));
        Self {
            trace_cache,
            action_buffer,
            settings,
            glyph_width,
        }
    }

    #[cfg(feature = "egui")]
    fn render_traces(&mut self, trace_ids: &[TraceId], ui: &mut egui::Ui) -> InnerResponse<()> {
        ui.allocate_at_least(Vec2::new(ui.available_width(), 0.), Sense::hover());
        ui.vertical(|ui| {
            for &trace_id in trace_ids {
                self.render_trace_view_tree(trace_id, ui)
            }
        })
    }

    fn render_trace_view_tree(&mut self, trace_id: TraceId, ui: &mut egui::Ui)
    where
        TraceProtocol: IsTraceProtocol,
    {
        let pedestal = self.trace_cache.pedestal();
        let entry = &self.trace_cache[trace_id];
        self.render_trace_view(pedestal, trace_id, entry, ui);
        if entry.expanded()
            && let Some(subtrace_ids) = entry.subtrace_ids()
        {
            self.render_subtraces(ui, entry.view_data().trace_kind, subtrace_ids);
        }
        for &associated_trace_id in entry.associated_trace_ids() {
            self.render_associated_trace(associated_trace_id, ui)
        }
    }

    fn render_trace_view(
        &mut self,
        pedestal: <TraceProtocol as IsTraceProtocol>::Pedestal,
        trace_id: TraceId,
        entry: &TraceSynchrotronEntry<TraceProtocol>,
        ui: &mut egui::Ui,
    ) where
        TraceProtocol: IsTraceProtocol,
    {
        let response = egui::Frame::none()
            .inner_margin(Margin {
                left: 1.0,
                right: 1.,
                top: 1.,
                bottom: 1.,
            })
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.;
                    // render prefix
                    let prefix_width = self.glyph_width * 2.8;
                    if entry.view_data().have_subtraces() {
                        let button_text = match entry.expanded() {
                            true => "-",
                            false => "+",
                        };
                        let button_text = RichText::new(button_text).family(FontFamily::Monospace);
                        let button = Button::new(button_text);
                        let button_response = button.ui(ui);
                        if button_response.clicked() {
                            self.action_buffer
                                .push(TraceViewAction::ToggleExpansion { trace_id })
                        };
                        ui.allocate_space(Vec2::new(
                            prefix_width - button_response.rect.width(),
                            button_response.rect.height(),
                        ));
                    } else {
                        ui.allocate_space(Vec2::new(prefix_width, 0.));
                    }
                    ui.vertical(|ui| {
                        let lines_data = entry.view_data().lines_data();
                        for line_data in &lines_data[..(lines_data.len() - 1)] {
                            ui.horizontal(|ui| self.render_line(line_data, trace_id, entry, ui));
                        }
                        ui.horizontal(|ui| {
                            self.render_line(lines_data.last().unwrap(), trace_id, entry, ui);
                            match entry.stalk(pedestal) {
                                TraceStalk::None => (),
                                TraceStalk::Val(value_control_flow) => match value_control_flow {
                                    ValControlFlow::Continue(value) => self.render_value(value, ui),
                                    ValControlFlow::LoopContinue => todo!(),
                                    ValControlFlow::LoopExit(_) => todo!(),
                                    ValControlFlow::Return(value) => {
                                        ui.label("return");
                                        self.render_value(value, ui)
                                    }
                                    ValControlFlow::Undefined => todo!(),
                                    ValControlFlow::Err(_) => todo!(),
                                },
                                TraceStalk::Vm(_) => todo!(),
                            }
                        });
                    })
                })
            })
            .response;
        if response.clicked() {
            todo!()
        }
    }

    fn render_subtraces(
        &mut self,
        ui: &mut egui::Ui,
        trace_kind: TraceKind,
        subtrace_ids: &[TraceId],
    ) {
        match trace_kind {
            TraceKind::Submodule => {
                egui::Frame::none()
                    .inner_margin(Margin {
                        left: 2.0,
                        right: 0.5,
                        top: 0.5,
                        bottom: 2.0,
                    })
                    .fill(Color32::from_rgb(14, 14, 14))
                    .show(ui, |ui| {
                        egui::Frame::none()
                            .fill(Color32::from_rgb(14, 14, 14))
                            .inner_margin(1.)
                            .show(ui, |ui| self.render_traces(subtrace_ids, ui))
                    });
            }
            TraceKind::EagerPatternExpr => todo!(),
            TraceKind::ValItem => {
                egui::Frame::none()
                    .fill(Color32::BLACK)
                    .inner_margin(1.)
                    .show(ui, |ui| self.render_traces(subtrace_ids, ui));
            }
            TraceKind::LazyCall => todo!(),
            TraceKind::LazyCallInput => todo!(),
            TraceKind::LazyExpr => todo!(),
            TraceKind::LazyPatternExpr => todo!(),
            TraceKind::LazyStmt => {
                self.render_traces(subtrace_ids, ui);
            }
            TraceKind::EagerCall => todo!(),
            TraceKind::EagerExpr => todo!(),
            TraceKind::EagerStmt => {
                self.render_traces(subtrace_ids, ui);
            }
            TraceKind::EagerCallInput => todo!(),
        }
    }

    fn render_associated_trace(&mut self, associated_trace_id: TraceId, ui: &mut egui::Ui) {
        egui::Frame::none()
            .inner_margin(3.0)
            .fill(Color32::from_rgb(84, 84, 84))
            .show(ui, |ui| {
                egui::Frame::none()
                    .fill(Color32::from_rgb(44, 44, 44))
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 0.;
                        ui.allocate_space(Vec2::new(ui.available_width(), 0.));
                        self.render_trace_view_tree(associated_trace_id, ui);
                    })
            });
    }

    fn render_line(
        &mut self,
        line_data: &TraceViewLineData,
        trace_id: TraceId,
        entry: &TraceSynchrotronEntry<TraceProtocol>,
        ui: &mut egui::Ui,
    ) {
        for token_data in line_data.tokens_data() {
            self.render_token(token_data, trace_id, entry, ui);
        }
    }

    fn render_token(
        &mut self,
        token_data: &TraceViewTokenData,
        trace_id: TraceId,
        entry: &TraceSynchrotronEntry<TraceProtocol>,
        ui: &mut egui::Ui,
    ) {
        let token_foreground_colors = self
            .settings
            .code_editor_settings()
            .token_foreground_colors();
        let spaces_before = token_data.spaces_before();
        self.render_space_chars(spaces_before, ui);
        let mut label = Label::new(
            RichText::new(token_data.text())
                .family(FontFamily::Monospace)
                .color(token_foreground_colors[token_data.token_class()]),
        );
        if token_data.associated_trace_id().is_some() {
            label = label.sense(Sense::click());
        }
        let label_response = label.ui(ui);
        if let Some(associated_trace_id) = token_data.associated_trace_id() {
            if label_response.clicked() {
                self.action_buffer
                    .push(TraceViewAction::ToggleAssociatedTrace {
                        trace_id,
                        associated_trace_id,
                    })
            }
            if entry.associated_trace_ids().contains(&associated_trace_id) {
                label_response.highlight();
            } else if label_response.hovered() {
                label_response.highlight();
            }
        }
    }

    fn render_value(&self, value: &ValuePresentation, ui: &mut egui::Ui)
    where
        TraceProtocol: IsTraceProtocol,
    {
        self.render_space_chars(1, ui);
        match value {
            ValuePresentation::Unit(()) => {
                ui.label("()");
            }
            ValuePresentation::Bool(b) => {
                ui.label("{b}");
            }
            ValuePresentation::Enum => todo!(),
            ValuePresentation::Struct => todo!(),
            ValuePresentation::AdHoc(s) => {
                // ad hoc
                ui.label(s);
            }
        }
    }

    /// here space means the char ` `
    fn render_space_chars(&self, n: u32, ui: &mut egui::Ui) {
        if n > 0 {
            ui.allocate_space(Vec2::new(self.glyph_width * (n as f32), 0.));
        }
    }
}

impl<'a, TraceProtocol, Settings> egui::Widget for TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    Settings: HasTraceViewDocSettings,
{
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        self.render_traces(self.trace_cache.root_trace_ids(), ui)
            .response
    }
}
