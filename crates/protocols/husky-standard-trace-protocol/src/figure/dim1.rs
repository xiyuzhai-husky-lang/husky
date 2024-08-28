use husky_standard_linket_impl::pedestal::StandardJointPedestal;

use crate::chart::StandardChartDim1;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct StandardFigureDim1 {
    data: Vec<StandardFigureDim0>,
}

/// # constructor
impl StandardFigureDim1 {
    pub(super) fn from_chart(
        chart: StandardChartDim1<CompositeVisual<TraceId>>,
        trace_plot_map: &TracePlotInfos,
        visual_synchrotron: &VisualSynchrotron,
    ) -> Self {
        Self {
            data: chart
                .into_iter()
                .map(|chart_dim0| {
                    StandardFigureDim0::from_chart(chart_dim0, trace_plot_map, visual_synchrotron)
                })
                .collect(),
        }
    }
}

/// # ui
impl StandardFigureDim1 {
    pub(super) fn ui(
        &self,
        visual_synchrotron: &VisualSynchrotron,
        cache: &mut FigureUiCache<Ui>,
        ui: &mut Ui,
    ) {
        let num_columns = 7;
        let l = ui.available_height().min(ui.available_width()) / (num_columns as f32);
        let l = l.floor();
        egui::Grid::new("generic_standard_figure")
            .num_columns(num_columns)
            .show(ui, |ui| {
                let num_rows = self.data.len() / num_columns
                    + if self.data.len() % num_columns == 0 {
                        0
                    } else {
                        1
                    };
                for i in 0..num_rows {
                    for j in 0..num_columns {
                        let index = i * num_columns + j;
                        if index < self.data.len() {
                            ui.allocate_ui(Vec2::splat(l), |ui| {
                                self.data[index].ui(visual_synchrotron, cache, ui)
                            });
                        }
                    }
                    ui.end_row()
                }
            });
    }
}
