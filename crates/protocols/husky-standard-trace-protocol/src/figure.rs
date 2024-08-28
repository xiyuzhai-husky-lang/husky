mod dim0;
mod dim1;
pub mod dim2;

use crate::chart::StandardChart;

use self::{dim0::StandardFigureDim0, dim1::StandardFigureDim1};
use egui::{pos2, Color32, Rect, Ui, Vec2};
use husky_ki_repr_interface::KiReprInterface;
use husky_linket_impl::pedestal::{IsPedestal, IsPedestalFull};
use husky_standard_linket_impl::pedestal::StandardPedestal;
use husky_standard_linket_impl::static_var::StandardStaticVarId;
use husky_trace_protocol::{
    chart::Chart,
    figure::{FigureUi, FigureUiCache, IsFigure},
    id::TraceId,
    server::TracePlotMap,
};
use husky_visual_protocol::visual::{
    shape::{Point, VisualRect},
    CompositeVisual,
};
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
    Void,
    Dim0(StandardFigureDim0),
    Dim1(StandardFigureDim1),
}

/// # impl IsFigure
impl IsFigure for StandardFigure {
    type Pedestal = StandardPedestal;

    fn from_chart(
        chart: Option<StandardChart<CompositeVisual<TraceId>>>,
        trace_plot_map: &TracePlotMap,
        visual_synchrotron: &VisualSynchrotron,
    ) -> Self {
        let Some(chart) = chart else {
            return StandardFigure::Void;
        };
        match chart {
            Chart::Dim0(chart) => {
                StandardFigureDim0::from_chart(chart, trace_plot_map, visual_synchrotron).into()
            }
            Chart::Dim1(chart) => todo!(),
            Chart::Dim2(chart) => todo!(),
        }
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
            StandardFigure::Void => (),
            StandardFigure::Dim0(slf) => slf.ui(visual_synchrotron, cache, ui),
            StandardFigure::Dim1(slf) => slf.ui(visual_synchrotron, cache, ui),
        }
    }
}
