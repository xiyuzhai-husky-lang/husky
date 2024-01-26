mod generic;
mod specific;
use self::{generic::GenericGraphics2dFigure, specific::SpecificGraphics2dFigure};
use egui::{pos2, Color32, Rect, Ui, Vec2};
use husky_task_interface::{
    pedestal::{IsPedestal, IsPedestalFull},
    val_repr::{ValDomainReprInterface, ValReprInterface},
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
pub enum Graphics2dFigure<Pedestal: IsPedestal> {
    Specific(SpecificGraphics2dFigure),
    Generic(GenericGraphics2dFigure<Pedestal>),
}

/// # impl IsFigure
impl<Pedestal: IsPedestalFull> IsFigure<Pedestal> for Graphics2dFigure<Pedestal> {
    fn new_specific(
        followed_visual: Option<(TraceId, ValReprInterface)>,
        accompanyings: &[(TraceId, ValReprInterface)],
        f: impl FnMut(ValReprInterface, &mut VisualSynchrotron) -> Visual,
        visual_synchrotron: &mut VisualSynchrotron,
    ) -> Self {
        SpecificGraphics2dFigure::new(followed_visual, accompanyings, f, visual_synchrotron).into()
    }

    fn new_generic(
        followed_visual: Option<(TraceId, ValReprInterface)>,
        accompanyings: &[(TraceId, ValReprInterface)],
        pedestals: impl Iterator<Item = Pedestal>,
        f: impl FnMut(ValReprInterface, Pedestal, &mut VisualSynchrotron) -> Visual,
        visual_synchrotron: &mut VisualSynchrotron,
    ) -> Self {
        GenericGraphics2dFigure::new(
            followed_visual,
            accompanyings,
            pedestals,
            f,
            visual_synchrotron,
        )
        .into()
    }
}

impl<Pedestal: IsPedestalFull> FigureUi<Ui> for Graphics2dFigure<Pedestal> {
    fn figure_ui(
        &self,
        visual_synchrotron: &VisualSynchrotron,
        cache: &mut FigureUiCache<Ui>,
        ui: &mut Ui,
    ) {
        match self {
            Graphics2dFigure::Specific(figure) => {
                figure.specific_figure_ui(visual_synchrotron, cache, ui)
            }
            Graphics2dFigure::Generic(figure) => {
                figure.generic_figure_ui(visual_synchrotron, cache, ui)
            }
        }
    }
}
