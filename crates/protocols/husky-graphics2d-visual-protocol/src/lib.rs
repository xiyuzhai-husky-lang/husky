pub mod action;

use husky_trace_protocol::{figure::IsFigure, id::TraceId};
use husky_visual_protocol::visual::{image::ImageVisual, Visual};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Graphics2dFigure {
    image: Option<ImageVisual>,
    shapes: Vec<()>,
}

impl IsFigure for Graphics2dFigure {
    fn new_specific(
        followed_visual: Option<(TraceId, Visual)>,
        accompanying_visuals: Vec<(TraceId, Visual)>,
    ) -> Self {
        Self {
            image: todo!(),
            shapes: todo!(),
        }
    }
}
