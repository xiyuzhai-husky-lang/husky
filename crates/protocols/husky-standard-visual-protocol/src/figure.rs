mod generic;
mod specific;
use self::{generic::GenericStandardFigure, specific::SpecificStandardFigure};
use egui::{pos2, Color32, Rect, Ui, Vec2};
use husky_devsoul_interface::{
    item_path::ItemPathIdInterface,
    ki_repr::{KiDomainReprInterface, KiReprInterface},
    pedestal::{IsPedestal, IsPedestalFull},
};
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
pub enum StandardFigure<Pedestal: IsPedestal> {
    Specific(SpecificStandardFigure),
    Generic(GenericStandardFigure<Pedestal>),
}

/// # impl IsFigure
impl<Pedestal: IsPedestalFull> IsFigure<Pedestal> for StandardFigure<Pedestal> {
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
        pedestals: impl Iterator<Item = Pedestal>,
        f: impl FnMut(KiReprInterface, Pedestal, &mut VisualSynchrotron) -> Visual,
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

impl<Pedestal: IsPedestalFull> FigureUi<Ui> for StandardFigure<Pedestal> {
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
