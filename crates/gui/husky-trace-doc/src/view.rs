use crate::*;
use egui::{
    Button, Color32, FontFamily, InnerResponse, Label, Margin, RichText, Sense, TextStyle,
    Vec2, Widget,
};

use husky_trace_protocol::{
    cache::{TraceCache, TraceCacheEntry},
    id::{TraceId, TraceKind},
    view::{action::TraceViewActionBuffer, TraceViewLineData, TraceViewTokenData},
};
use husky_visual_protocol::IsVisualComponent;

pub(crate) struct TraceDocView<'a, VisualComponent, Settings>
where
    VisualComponent: IsVisualComponent,
    Settings: HasTraceViewDocSettings,
{
    trace_cache: &'a TraceCache<VisualComponent>,
    action_buffer: &'a mut TraceViewActionBuffer<VisualComponent>,
    settings: &'a mut Settings,
    // cached values
    glyph_width: f32,
    // trace_listing: Vec<TraceId>,
}

impl<'a, VisualComponent, Settings> TraceDocView<'a, VisualComponent, Settings>
where
    VisualComponent: IsVisualComponent,
    Settings: HasTraceViewDocSettings,
{
    pub(crate) fn new(
        trace_cache: &'a TraceCache<VisualComponent>,
        action_buffer: &'a mut TraceViewActionBuffer<VisualComponent>,
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
        use egui::{Vec2};
        ui.allocate_at_least(Vec2::new(ui.available_width(), 0.), Sense::hover());
        ui.vertical(|ui| {
            for &trace_id in trace_ids {
                self.render_trace_view_tree(trace_id, ui)
            }
        })
    }

    fn render_trace_view_tree(&mut self, trace_id: TraceId, ui: &mut egui::Ui)
    where
        VisualComponent: IsVisualComponent,
    {
        let entry = &self.trace_cache[trace_id];
        self.render_trace_view(trace_id, entry, ui);
        if entry.expanded()
            && let Some(subtrace_ids) = entry.subtrace_ids()
        {
            self.render_subtraces(ui, trace_id, subtrace_ids);
        }
        for &associated_trace_id in entry.associated_trace_ids() {
            self.render_associated_trace(associated_trace_id, ui)
        }
    }

    fn render_trace_view(&mut self, trace_id: TraceId, entry: &TraceCacheEntry, ui: &mut egui::Ui)
    where
        VisualComponent: IsVisualComponent,
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
                            self.glyph_width * 2.8 - button_response.rect.width(),
                            button_response.rect.height(),
                        ));
                    } else {
                        ui.allocate_space(Vec2::new(self.glyph_width * 2.8, 0.));
                    }
                    ui.vertical(|ui| {
                        for line_data in entry.view_data().lines_data() {
                            self.render_line(line_data, trace_id, entry, ui)
                        }
                    })
                })
            })
            .response;
        if response.clicked() {
            todo!()
        }
    }

    fn render_subtraces(&mut self, ui: &mut egui::Ui, trace_id: TraceId, subtrace_ids: &[TraceId]) {
        match trace_id.kind() {
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
            TraceKind::ValItem => {
                egui::Frame::none()
                    .fill(Color32::BLACK)
                    .inner_margin(1.)
                    .show(ui, |ui| self.render_traces(subtrace_ids, ui));
            }
            TraceKind::LazyCall => todo!(),
            TraceKind::LazyExpr => todo!(),
            TraceKind::LazyStmt => {
                self.render_traces(subtrace_ids, ui);
            }
            TraceKind::EagerCall => todo!(),
            TraceKind::EagerExpr => todo!(),
            TraceKind::EagerStmt => {
                self.render_traces(subtrace_ids, ui);
            }
        }
    }

    fn render_associated_trace(&mut self, associated_trace_id: TraceId, ui: &mut egui::Ui) {
        egui::Frame::none()
            .inner_margin(1.0)
            .fill(Color32::DARK_GREEN)
            .show(ui, |ui| {
                egui::Frame::none()
                    .fill(Color32::from_rgb(14, 14, 14))
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
        entry: &TraceCacheEntry,
        ui: &mut egui::Ui,
    ) {
        ui.horizontal(|ui| {
            for token_data in line_data.tokens_data() {
                self.render_token(token_data, trace_id, entry, ui);
            }
        });
    }

    fn render_token(
        &mut self,
        token_data: &TraceViewTokenData,
        trace_id: TraceId,
        entry: &TraceCacheEntry,
        ui: &mut egui::Ui,
    ) {
        
        let token_foreground_colors = self
            .settings
            .code_editor_settings()
            .token_foreground_colors();
        let spaces_before = token_data.spaces_before();
        if spaces_before > 0 {
            ui.allocate_space(Vec2::new(self.glyph_width * (spaces_before as f32), 0.));
        }
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
}

impl<'a, VisualComponent, Settings> egui::Widget for TraceDocView<'a, VisualComponent, Settings>
where
    VisualComponent: IsVisualComponent,
    Settings: HasTraceViewDocSettings,
{
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        self.render_traces(self.trace_cache.root_trace_ids(), ui)
            .response
    }
}
