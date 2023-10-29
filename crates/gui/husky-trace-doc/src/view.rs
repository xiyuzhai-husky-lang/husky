use crate::*;
use egui::{Color32, FontFamily, InnerResponse, Label, RichText, Sense, Stroke, TextStyle, Widget};
use husky_trace_protocol::{
    cache::{TraceCache, TraceCacheEntry},
    id::TraceId,
    view::action::TraceViewActionBuffer,
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
        use egui::{InnerResponse, Margin};
        ui.vertical(|ui| {
            for &trace_id in trace_ids {
                let entry = &self.trace_cache[trace_id];
                self.render_trace(trace_id, entry, ui);
                if entry.expanded()
                    && let Some(subtrace_ids) = entry.subtrace_ids()
                {
                    egui::Frame::none()
                        .inner_margin(Margin {
                            left: 22.0,
                            right: 0.,
                            top: 0.,
                            bottom: 0.,
                        })
                        .show(ui, |ui| self.render_traces(subtrace_ids, ui));
                }
            }
        })
    }

    fn render_trace(&mut self, trace_id: TraceId, entry: &TraceCacheEntry, ui: &mut egui::Ui)
    where
        VisualComponent: IsVisualComponent,
    {
        use husky_trace_protocol::view::SeparationAfter;

        let token_foreground_colors = self
            .settings
            .code_editor_settings()
            .token_foreground_colors();
        ui.horizontal(|ui| {
            let button_text = match entry.expanded() {
                true => "-",
                false => "+",
            };
            let button_text = RichText::new(button_text).family(FontFamily::Monospace);
            if ui.button(button_text).clicked() {
                self.action_buffer
                    .push(TraceViewAction::ToggleExpansion { trace_id })
            };
            let mut new_line = false;
            for token_data in entry.view_data().tokens_data() {
                if new_line {
                    todo!()
                }
                ui.spacing_mut().item_spacing.x = match token_data.separation_after() {
                    SeparationAfter::SameLine { spaces } => (spaces as f32) * self.glyph_width,
                    SeparationAfter::NextLine { indent } => {
                        new_line = true;
                        0.
                    }
                    SeparationAfter::Eof => 0.,
                };
                let label_response = Label::new(
                    RichText::new(token_data.text())
                        .family(FontFamily::Monospace)
                        .color(token_foreground_colors[token_data.token_class()]),
                )
                .ui(ui);
                if label_response.hovered() {
                    label_response.highlight();
                }
            }
        });
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
