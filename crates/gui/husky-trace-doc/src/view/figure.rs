use super::*;
use egui::{load::Bytes, Frame, Image, ImageSource, Rect};

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    Settings: HasTraceViewDocSettings,
{
    pub(super) fn render_figure(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let (id, rect) = ui.allocate_space(vec2(300.0, 300.));
            let painter = &ui.painter_at(rect);
            painter.rect(
                Rect::from_two_pos(rect.min, rect.min + vec2(150.0, 150.0)),
                5.0,
                Color32::LIGHT_RED,
                (3.0, Color32::DARK_GRAY),
            );
            painter.rect(
                Rect::from_two_pos(rect.min, rect.min + vec2(100.0, 100.0)),
                5.0,
                Color32::RED,
                (3.0, Color32::DARK_GRAY),
            );
            painter.rect(
                Rect::from_two_pos(rect.min, rect.min + vec2(50.0, 50.0)),
                5.0,
                Color32::BROWN,
                (3.0, Color32::DARK_GRAY),
            );
        });
        // ui.allocate_space(ui.available_size())
    }
}
