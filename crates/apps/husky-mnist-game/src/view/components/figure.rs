use crate::{
    channel::MnistChannel,
    op::time::OpTime,
    trace::{Trace, TraceSelection},
    ui_cache::MnistUiCache,
    MnistDb,
};
use egui::Frame;
use husky_graphics2d_visual_protocol::figure::Graphics2dFigure;
use husky_ml_task_interface::pedestal::MlPedestal;
use husky_trace_protocol::figure::IsFigure;
use husky_visual_protocol::synchrotron::VisualSynchrotron;

pub struct FigureView<'a> {
    trace_selection: &'a TraceSelection,
    db: &'a MnistDb,
    ui_cache: &'a mut MnistUiCache,
}

pub type Figure = Graphics2dFigure<MlPedestal>;

impl MnistChannel {
    pub(in super::super) fn figure_view<'a>(
        &'a self,
        db: &'a MnistDb,
        ui_cache: &'a mut MnistUiCache,
    ) -> FigureView<'a> {
        FigureView {
            trace_selection: self.trace_selection(),
            db,
            ui_cache,
        }
    }

    pub(in super::super) fn figure<'a>(
        &'a self,
        pedestal: MlPedestal,
        op_time: OpTime,
        db: &'a MnistDb,
        visual_synchrotron: &mut VisualSynchrotron,
    ) -> Figure {
        let accompanyings_expect_followed = self
            .trace_selection()
            .set()
            .iter()
            .map(|trace| (trace.into(), trace.into()))
            .collect::<Vec<_>>();
        match pedestal {
            MlPedestal::Specific(input_id) => {
                let op_snap = db.op_snap(input_id, op_time);
                Figure::new_specific(
                    None,
                    &accompanyings_expect_followed,
                    |val_repr_interface, visual_synchrotron| {
                        let trace: Trace = val_repr_interface.into();
                        trace.visual(db, op_snap)
                    },
                    visual_synchrotron,
                )
            }
            MlPedestal::Generic => todo!(),
        }
    }
}

impl<'a> FigureView<'a> {}

impl<'a> egui::Widget for FigureView<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        Frame::none()
            .show(ui, |ui| {
                for trace in self.trace_selection.set().iter() {
                    trace.visual(self.db, todo!());
                }
            })
            .response
    }
}
