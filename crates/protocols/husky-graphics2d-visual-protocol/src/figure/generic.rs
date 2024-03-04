use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct GenericGraphics2dFigure<Pedestal: IsPedestal> {
    specifics: Vec<(Pedestal, SpecificGraphics2dFigure)>,
}

/// # constructor
impl<Pedestal: IsPedestalFull> GenericGraphics2dFigure<Pedestal> {
    pub(crate) fn new(
        followed_visual: Option<(TraceId, KiReprInterface)>,
        accompanyings: &[(TraceId, KiReprInterface)],
        pedestals: impl Iterator<Item = Pedestal>,
        mut f: impl FnMut(KiReprInterface, Pedestal, &mut VisualSynchrotron) -> Visual,
        visual_synchrotron: &mut VisualSynchrotron,
    ) -> Self {
        GenericGraphics2dFigure {
            specifics: pedestals
                .map(|pedestal| {
                    (
                        pedestal,
                        SpecificGraphics2dFigure::new(
                            followed_visual
                                .map(|(trace_id, ki_repr_interface)| (trace_id, ki_repr_interface)),
                            accompanyings,
                            |ki_repr_interface, visual_synchrotron| {
                                f(ki_repr_interface, pedestal, visual_synchrotron)
                            },
                            visual_synchrotron,
                        ),
                    )
                })
                .collect(),
        }
    }
}

/// # ui
impl<Pedestal: IsPedestalFull> GenericGraphics2dFigure<Pedestal> {
    pub(super) fn generic_figure_ui(
        &self,
        visual_synchrotron: &VisualSynchrotron,
        cache: &mut FigureUiCache<Ui>,
        ui: &mut Ui,
    ) {
        let num_columns = 7;
        let l = ui.available_height().min(ui.available_width()) / (num_columns as f32);
        let l = l.floor();
        egui::Grid::new("generic_graphics2d_figure")
            .num_columns(num_columns)
            .show(ui, |ui| {
                let num_rows = self.specifics.len() / num_columns
                    + if self.specifics.len() % num_columns == 0 {
                        0
                    } else {
                        1
                    };
                for i in 0..num_rows {
                    for j in 0..num_columns {
                        let index = i * num_columns + j;
                        if index < self.specifics.len() {
                            ui.allocate_ui(Vec2::splat(l), |ui| {
                                self.specifics[index].1.specific_figure_ui(
                                    visual_synchrotron,
                                    cache,
                                    ui,
                                )
                            });
                        }
                    }
                    ui.end_row()
                }
            });
    }
}
