use crate::{channel::MnistChannel, trace::TraceSelection, ui_cache::MnistUiCache, MnistDb};
use egui::Frame;
use husky_graphics2d_visual_protocol::figure::Graphics2dFigure;
use husky_ml_task_interface::pedestal::MlPedestal;

pub struct FigureView<'a> {
    trace_selection: &'a TraceSelection,
    db: &'a MnistDb,
    ui_cache: &'a mut MnistUiCache,
}

impl MnistChannel {
    pub(in super::super) fn figure<'a>(
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

pub type Figure = Graphics2dFigure<MlPedestal>;
