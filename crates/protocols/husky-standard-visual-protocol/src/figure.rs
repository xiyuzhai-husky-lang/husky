mod generic;
mod specific;

use self::{generic::GenericStandardFigure, specific::SpecificStandardFigure};
use egui::{pos2, Color32, Rect, Ui, Vec2};
use husky_ki_repr_interface::KiReprInterface;
use husky_linket_impl::pedestal::{IsPedestal, IsPedestalFull};
use husky_standard_linket_impl::pedestal::StandardPedestal;
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
pub enum StandardFigure {
    Specific(SpecificStandardFigure),
    Generic(GenericStandardFigure),
}

/// # impl IsFigure
impl IsFigure<StandardPedestal> for StandardFigure {
    fn new_specific(
        followed_visual: Option<(TraceId, KiReprInterface)>,
        accompanyings: &[(TraceId, KiReprInterface)],
        f: impl FnMut(KiReprInterface, &mut VisualSynchrotron) -> Visual,
        visual_synchrotron: &mut VisualSynchrotron,
    ) -> Self {
        SpecificStandardFigure::new(followed_visual, accompanyings, f, visual_synchrotron).into()
    }

    fn new_generic(
        followed_visual: Option<(TraceId, KiReprInterface)>,
        accompanyings: &[(TraceId, KiReprInterface)],
        pedestals: impl Iterator<Item = StandardPedestal>,
        f: impl FnMut(KiReprInterface, StandardPedestal, &mut VisualSynchrotron) -> Visual,
        visual_synchrotron: &mut VisualSynchrotron,
    ) -> Self {
        GenericStandardFigure::new(
            followed_visual,
            accompanyings,
            pedestals,
            f,
            visual_synchrotron,
        )
        .into()
    }
}

impl FigureUi<Ui> for StandardFigure {
    fn figure_ui(
        &self,
        visual_synchrotron: &VisualSynchrotron,
        cache: &mut FigureUiCache<Ui>,
        ui: &mut Ui,
    ) {
        match self {
            StandardFigure::Specific(figure) => {
                figure.specific_figure_ui(visual_synchrotron, cache, ui)
            }
            StandardFigure::Generic(figure) => {
                figure.generic_figure_ui(visual_synchrotron, cache, ui)
            }
        }
    }
}
