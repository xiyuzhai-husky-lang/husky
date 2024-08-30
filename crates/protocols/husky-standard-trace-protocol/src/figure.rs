mod dim0;
mod dim1;
pub mod dim2;

use crate::chart::StandardChart;

use self::{dim0::StandardFigureDim0, dim1::StandardFigureDim1};
#[cfg(feature = "egui")]
use egui::{pos2, Color32, Rect, Ui, Vec2};
use husky_ki_repr_interface::KiReprInterface;
use husky_linket_impl::pedestal::{IsPedestal, IsPedestalFull};
use husky_standard_linket_impl::pedestal::StandardPedestal;
use husky_standard_linket_impl::static_var::StandardVarId;
use husky_trace_protocol::{
    chart::Chart,
    figure::{FigureUi, IsFigure},
    trace_id::TraceId,
    server::TracePlotInfos,
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
use ui::visual::cache::VisualUiCache;

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
        trace_plot_map: &TracePlotInfos,
        visual_synchrotron: &VisualSynchrotron,
    ) -> Self {
        let Some(chart) = chart else {
            return StandardFigure::Void;
        };
        match chart {
            Chart::Dim0(chart) => {
                StandardFigureDim0::from_chart(chart, trace_plot_map, visual_synchrotron).into()
            }
            Chart::Dim1(chart) => {
                StandardFigureDim1::from_chart(chart, trace_plot_map, visual_synchrotron).into()
            }
            Chart::Dim2(chart) => todo!(),
        }
    }
}

#[cfg(feature = "egui")]
impl FigureUi<Ui> for StandardFigure {
    fn ui(
        &self,
        rect: ::egui::Rect,
        visual_synchrotron: &VisualSynchrotron,
        cache: &mut VisualUiCache<Ui>,
        ui: &mut Ui,
    ) {
        match self {
            StandardFigure::Void => (),
            StandardFigure::Dim0(slf) => slf.ui(rect, visual_synchrotron, cache, ui),
            StandardFigure::Dim1(slf) => slf.ui(rect, visual_synchrotron, cache, ui),
        }
    }
}
