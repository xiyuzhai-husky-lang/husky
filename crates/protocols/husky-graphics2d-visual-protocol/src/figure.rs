mod generic;
mod specific;
use self::{generic::GenericGraphics2dFigure, specific::SpecificGraphics2dFigure};
use egui::{pos2, vec2, Color32, Rect, Ui, Vec2};
use husky_trace_protocol::{
    figure::{FigureUi, FigureUiCache, IsFigure},
    id::TraceId,
};
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

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Graphics2dFigure {
    Specific(SpecificGraphics2dFigure),
    Generic(GenericGraphics2dFigure),
}

impl IsFigure for Graphics2dFigure {
    fn new_specific(
        followed_visual: Option<(TraceId, Visual)>,
        accompanying_visuals: impl IntoIterator<Item = (TraceId, Visual)>,
        visual_synchrotron: &VisualSynchrotron,
    ) -> Self {
        SpecificGraphics2dFigure::new(
            followed_visual,
            accompanying_visuals.into_iter(),
            visual_synchrotron,
        )
        .into()
    }

    fn new_generic() -> Self {
        GenericGraphics2dFigure::new().into()
    }
}

impl FigureUi<Ui> for Graphics2dFigure {
    fn figure_ui(
        &self,
        visual_synchrotron: &VisualSynchrotron,
        cache: &mut FigureUiCache<Ui>,
        ui: &mut Ui,
    ) {
        match self {
            Graphics2dFigure::Specific(figure) => figure.figure_ui(visual_synchrotron, cache, ui),
            Graphics2dFigure::Generic(_) => (),
        }
    }
}
