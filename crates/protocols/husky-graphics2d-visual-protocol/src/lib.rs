pub mod action;

use egui::{pos2, Rect};
use husky_trace_protocol::{figure::IsFigure, id::TraceId};
use husky_visual_protocol::{
    synchrotron::VisualSynchrotron,
    visual::{image::ImageVisual, shape::ShapeVisual, Visual},
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
        accompanying_visuals: impl Iterator<Item = (TraceId, Visual)>,
    ) -> Self {
        let mut images: Vec<ImageVisual> = vec![];
        let mut shapes: Vec<ShapeVisual> = vec![];
        followed_visual
            .into_iter()
            .chain(accompanying_visuals)
            .for_each(|(trace_id, visual)| match visual {
                Visual::Void => (),
                Visual::Primitive(_) => todo!(),
                Visual::Text(_) => todo!(),
                Visual::RichText(_) => todo!(),
                Visual::Image(image) => images.push(image),
                Visual::Shape(_) => todo!(),
                Visual::Mesh(_) => todo!(),
                Visual::Video(_) => todo!(),
            });
        Self { images, shapes }
    }
}

impl ui::visual_widget::VisualWidget<egui::Ui> for Graphics2dFigure {
    fn ui(&self, visual_synchrotron: &VisualSynchrotron, cache: &mut UiCache, ui: &mut egui::Ui) {
        for &image in &self.images {
            let (_, rect) = ui.allocate_space(ui.available_size());
            ui.painter().image(
                cache.texture_id(image, visual_synchrotron, ui),
                rect,
                Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                egui::Color32::LIGHT_BLUE,
            )
        }
        for shape in &self.shapes {
            todo!()
        }
    }
}
