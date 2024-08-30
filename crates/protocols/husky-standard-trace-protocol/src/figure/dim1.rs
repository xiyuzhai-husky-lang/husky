use super::*;
use crate::chart::StandardChartDim1;
use egui::Sense;
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
    pub(super) fn ui(
        &self,
        rect: ::egui::Rect,
        visual_synchrotron: &VisualSynchrotron,
        cache: &mut ui::visual::cache::VisualUiCache<Ui>,
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
                            // let (rect, response) =
                            //     ui.allocate_exact_size(Vec2::splat(l), Sense::hover());
                            // ui.allocate_ui_at_rect(rect, |ui| {
                            //     self.data[index].ui(rect, visual_synchrotron, cache, ui)
                            // });

                            // previous:
                            let (rect, response) =
                                ui.allocate_exact_size(Vec2::splat(0.0), Sense::hover());
                            ui.allocate_ui(Vec2::splat(l), |ui| {
                                self.data[index].ui(rect, visual_synchrotron, cache, ui)
                            });
                        }
                    }
                    ui.end_row()
                }
            });
    }
}
