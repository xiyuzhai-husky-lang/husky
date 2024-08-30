use super::*;
use crate::chart::StandardChartDim1;
use egui::{vec2, Frame, Sense};
use husky_standard_linket_impl::pedestal::StandardJointPedestal;
use ui::visual::cache::VisualUiCache;

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
#[cfg(feature = "egui")]
impl StandardFigureDim1 {
    pub(super) fn figure_ui(
        &self,
        visual_synchrotron: &VisualSynchrotron,
        cache: &mut ui::visual::cache::VisualUiCache<Ui>,
        ui: &mut Ui,
    ) {
        let num_rows = 7;
        let num_columns = 7;
        let grid_height = ui.available_height() / (num_rows as f32);
        let figure_height = grid_height.floor();
        let grid_width = ui.available_width() / (num_columns as f32);
        let figure_width = grid_width.floor();
        let base = ui.cursor().min;
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
                            let rect = Rect::from_min_max(
                                base + vec2((i as f32) * grid_width, (j as f32) * grid_height),
                                base + vec2(
                                    ((i + 1) as f32) * grid_width,
                                    ((j + 1) as f32) * grid_height,
                                ),
                            );
                            ui.allocate_ui_at_rect(rect, |ui| {
                                self.data[index].figure_ui(visual_synchrotron, cache, ui)
                            });
                        }
                    }
                    ui.end_row()
                }
            });
    }
}
