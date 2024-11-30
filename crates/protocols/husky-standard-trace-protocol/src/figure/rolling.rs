use super::*;
use husky_figure_zone_protocol::chunk_base::{
    text::{FigureTextChunkBaseData, FigureTextChunkBaseId},
    FigureChunkBase,
};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct RollingFigure {
    chunks: Vec<RollingFigureChunk>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct RollingFigureChunk {
    base: FigureChunkBase,
    specific_figures: Vec<SpecificFigure>,
}

impl RollingFigure {
    pub(super) fn from_chart(
        chart: Option<StandardChartDim1<CompositeVisual<TraceId>>>,
        trace_plot_map: &TracePlotInfos,
        visual_synchrotron: &mut VisualSynchrotron,
        f: impl Fn(u32, &mut VisualSynchrotron) -> FigureChunkBase,
    ) -> Self {
        use itertools::Itertools;

        let Some(chart) = chart else {
            return Self { chunks: vec![] };
        };
        let mut chunks = chart
            .points
            .into_iter()
            .group_by(|(var_id, _)| {
                let StandardVarId::Pair([chunk_id, _]) = *var_id else {
                    unreachable!()
                };
                chunk_id
            })
            .into_iter()
            .map(|(chunk_id, group)| {
                // TODO: other kinds of chunks
                let base = f(chunk_id, visual_synchrotron);
                RollingFigureChunk {
                    base,
                    specific_figures: group
                        .map(|(var_id, chart_dim0)| {
                            SpecificFigure::from_chart(
                                chart_dim0,
                                trace_plot_map,
                                visual_synchrotron,
                            )
                        })
                        .collect(),
                }
            })
            .collect();
        Self { chunks }
    }
}

impl RollingFigure {
    pub(super) fn for_all_joint_pedestals(&self, mut f: impl FnMut(&StandardJointPedestal)) {
        self.chunks
            .iter()
            .for_each(|chunk| chunk.for_all_joint_pedestals(&mut f))
    }
}

impl RollingFigureChunk {
    pub(super) fn for_all_joint_pedestals(&self, mut f: impl FnMut(&StandardJointPedestal)) {
        self.specific_figures
            .iter()
            .for_each(|figure| figure.for_all_joint_pedestals(&mut f))
    }
}

impl RollingFigure {
    pub(crate) fn figure_ui(
        &self,
        visual_synchrotron: &VisualSynchrotron,
        cache: &mut VisualUiCache<Ui>,
        ui: &mut Ui,
    ) {
        for chunk in self.chunks.iter() {
            // TODO: wrap in a scrollable area or something
            match chunk.base {
                FigureChunkBase::Text(figure_text_chunk_base_id) => {
                    let data = &visual_synchrotron[figure_text_chunk_base_id];
                    ui.label(&data.text);
                }
                _ => unreachable!(),
            }
        }
    }
}
