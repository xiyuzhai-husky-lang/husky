use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct StandardFigureDim1 {
    data: Vec<(StandardPedestal, StandardFigureDim0)>,
}

/// # constructor
impl StandardFigureDim1 {
    pub(crate) fn new(
        followed_visual: Option<(TraceId, KiReprInterface)>,
        accompanyings: &[(TraceId, KiReprInterface)],
        joint_pedestals: impl Iterator<Item = StandardPedestal>,
        mut f: impl FnMut(KiReprInterface, StandardPedestal, &mut VisualSynchrotron) -> Visual,
        visual_synchrotron: &mut VisualSynchrotron,
    ) -> Self {
        todo!()
        // StandardFigureDim1 {
        //     data: joint_pedestals
        //         .map(|pedestal| {
        //             let specific_standard_figure = StandardFigureDim0::new(
        //                 pedestal,
        //                 followed_visual
        //                     .map(|(trace_id, ki_repr_interface)| (trace_id, ki_repr_interface)),
        //                 accompanyings,
        //                 todo!(),
        //                 |ki_repr_interface, visual_synchrotron| {
        //                     f(ki_repr_interface, pedestal.clone(), visual_synchrotron)
        //                 },
        //                 visual_synchrotron,
        //             );
        //             (pedestal, specific_standard_figure)
        //         })
        //         .collect(),
        // }
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
                                self.data[index].1.ui(visual_synchrotron, cache, ui)
                            });
                        }
                    }
                    ui.end_row()
                }
            });
    }
}
