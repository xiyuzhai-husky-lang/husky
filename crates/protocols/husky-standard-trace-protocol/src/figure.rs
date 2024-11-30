pub mod dim2;
mod parading;
pub mod rolling;
mod specific;

use self::{parading::ParadingFigure, specific::SpecificFigure};
use crate::chart::{StandardChart, StandardChartDim0, StandardChartDim1};
#[cfg(feature = "egui")]
use egui::{pos2, Color32, Rect, Ui, Vec2};
use husky_figure_zone_protocol::{
    chunk_base::{text::FigureTextChunkBaseId, FigureChunkBase},
    FigureZone,
};
use husky_ki_repr_interface::KiReprInterface;
use husky_linket_impl::pedestal::{IsPedestal, IsPedestalFull, JointPedestal};
use husky_standard_linket_impl::pedestal::{StandardJointPedestal, StandardPedestal};
use husky_standard_linket_impl::var::StandardVarId;
use husky_trace_protocol::{
    chart::Chart,
    figure::{FigureUi, IsFigure},
    server::TracePlotInfos,
    trace_id::TraceId,
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
use rolling::RollingFigure;
use serde::{Deserialize, Serialize};
use ui::visual::cache::VisualUiCache;

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum StandardFigure {
    Void,
    Specific(SpecificFigure),
    Marching(ParadingFigure),
    Rolling(RollingFigure),
}

/// # impl IsFigure
impl IsFigure for StandardFigure {
    type Pedestal = StandardPedestal;

    fn for_all_joint_pedestals(&self, f: impl FnMut(&StandardJointPedestal)) {
        match self {
            StandardFigure::Void => (),
            StandardFigure::Specific(slf) => slf.for_all_joint_pedestals(f),
            StandardFigure::Marching(slf) => slf.for_all_joint_pedestals(f),
            StandardFigure::Rolling(slf) => slf.for_all_joint_pedestals(f),
        }
    }

    fn new_void() -> Self {
        Self::Void
    }

    fn new_specific(
        chart: StandardChartDim0<CompositeVisual<TraceId>>,
        trace_plot_map: &TracePlotInfos,
        visual_synchrotron: &VisualSynchrotron,
    ) -> StandardFigure {
        SpecificFigure::from_chart(chart, trace_plot_map, visual_synchrotron).into()
    }

    fn new_parading(
        chart: StandardChartDim1<CompositeVisual<TraceId>>,
        trace_plot_map: &TracePlotInfos,
        visual_synchrotron: &VisualSynchrotron,
    ) -> StandardFigure {
        ParadingFigure::from_chart(chart, trace_plot_map, visual_synchrotron).into()
    }

    fn new_rolling(
        chart: Option<StandardChartDim1<CompositeVisual<TraceId>>>,
        trace_plot_map: &TracePlotInfos,
        visual_synchrotron: &mut VisualSynchrotron,
        f: impl Fn(u32, &mut VisualSynchrotron) -> FigureChunkBase,
    ) -> StandardFigure {
        RollingFigure::from_chart(chart, trace_plot_map, visual_synchrotron, f).into()
    }
}

#[cfg(feature = "egui")]
impl FigureUi<Ui> for StandardFigure {
    fn figure_ui(
        &self,
        visual_synchrotron: &VisualSynchrotron,
        cache: &mut VisualUiCache<Ui>,
        ui: &mut Ui,
    ) {
        match self {
            StandardFigure::Void => (),
            StandardFigure::Specific(slf) => slf.figure_ui(visual_synchrotron, cache, ui),
            StandardFigure::Marching(slf) => slf.figure_ui(visual_synchrotron, cache, ui),
            StandardFigure::Rolling(slf) => slf.figure_ui(visual_synchrotron, cache, ui),
        }
    }
}
