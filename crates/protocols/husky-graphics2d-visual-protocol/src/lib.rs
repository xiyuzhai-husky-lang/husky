pub mod action;
mod specific;

use egui::{pos2, vec2, Color32, Rect, Vec2};
use husky_trace_protocol::{figure::IsFigure, id::TraceId};
use husky_visual_protocol::visual::shape::{Point, VisualRect};
use husky_visual_protocol::{
    synchrotron::VisualSynchrotron,
    visual::{
        image::ImageVisual,
        shape::{ShapeVisual, ShapeVisualData},
        Visual,
    },
};
use serde::{Deserialize, Serialize};
use ui::ui::egui::UiCache;

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Graphics2dFigure {
    images: Vec<ImageVisual>,
    shapes: Vec<ShapeVisual>,
}

impl IsFigure for Graphics2dFigure {
    fn new_specific(
        followed_visual: Option<(TraceId, Visual)>,
        accompanying_visuals: impl IntoIterator<Item = (TraceId, Visual)>,
        visual_synchrotron: &VisualSynchrotron,
    ) -> Self {
        specific::new_specific_figure(
            followed_visual,
            accompanying_visuals.into_iter(),
            visual_synchrotron,
        )
    }
}

impl ui::visual_widget::VisualWidget<egui::Ui> for Graphics2dFigure {
    fn ui(&self, visual_synchrotron: &VisualSynchrotron, cache: &mut UiCache, ui: &mut egui::Ui) {
        let x = ui.available_width() * 0.8;
        let y = ui.available_height() / 2.;
        let d = x.min(y);
        let (_, rect) = ui.allocate_space(Vec2::splat(d));
        for &image in &self.images {
            ui.painter().image(
                cache.texture_id(image, visual_synchrotron, ui),
                rect,
                Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            )
        }
        let mnist_visual_rect = VisualRect::mnist();
        let t = |point: Point| point.to_screen(mnist_visual_rect, rect);
        for shape in &self.shapes {
            use husky_print_utils::p;
            match shape.data(visual_synchrotron) {
                &ShapeVisualData::LineSegment { start, end } => ui
                    .painter()
                    .line_segment([t(start), t(end)], (2.0, Color32::YELLOW)),
                ShapeVisualData::Contour { points } => {
                    for i in 0..(points.len() - 1) {
                        ui.painter()
                            .line_segment([t(points[i]), t(points[i + 1])], (2.0, Color32::YELLOW))
                    }
                }
            }
        }
    }
}
