pub mod action;

use husky_trace_protocol::{figure::IsFigure, id::TraceId};
use husky_visual_protocol::{
    synchrotron::VisualSynchrotron,
    visual::{image::ImageVisual, shape::ShapeVisual, Visual},
};
use serde::{Deserialize, Serialize};

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
