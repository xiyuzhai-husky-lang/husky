use vec_like::VecSet;

use super::*;

impl DeveloperGuiContext {
    pub(super) fn receive_figure_canvases(
        &self,
        scope: Scope<'static>,
        new_figure_canvases: impl Iterator<Item = (FigureCanvasKey, &'static FigureCanvasData)>,
    ) {
        let mut figure_canvases = self.figure_canvases.borrow_mut(file!(), line!());
        for (key, data) in new_figure_canvases {
            insert_new!(figure_canvases, key, data);
        }
    }
    pub(super) fn receive_figure_controls(
        &self,
        scope: Scope<'static>,
        new_figure_controls: impl Iterator<Item = (FigureControlKey, FigureControlData)>,
    ) {
        let mut figure_controls = self.figure_controls.borrow_mut(file!(), line!());
        for (key, data) in new_figure_controls {
            assert!(figure_controls
                .insert(key, create_signal(scope, data))
                .is_none());
        }
    }

    pub(crate) fn new_figure_canvas_key(
        &self,
        trace: &TraceData,
        presentation: &Presentation,
        is_specific: bool,
    ) -> FigureCanvasKey {
        FigureCanvasKey::new(trace.kind, trace.id, presentation, is_specific)
    }

    pub(crate) fn figure_canvas_data(
        &self,
        trace: &TraceData,
        presentation: &Presentation,
    ) -> &'static FigureCanvasData {
        let figure_canvas_key =
            self.new_figure_canvas_key(trace, presentation, presentation.is_specific());
        let figure_canvases_borrowed = self.figure_canvases.borrow(file!(), line!());
        if let Some(figure_canvas_data) = figure_canvases_borrowed.get(&figure_canvas_key) {
            figure_canvas_data
        } else {
            log::info!("presentation = {presentation:?}");
            log::info!("figure canvases: {figure_canvases_borrowed:?}");
            log::info!("no entry with key {figure_canvas_key:?}");
            panic!()
        }
    }

    fn set_figure_control_data(
        &self,
        scope: Scope<'static>,
        key: FigureControlKey,
        figure_control_data: FigureControlData,
    ) {
        let opt_figure_control_signal = {
            let figure_controls = &mut self.figure_controls.borrow_mut(file!(), line!());
            if let Some(figure_control_signal) = figure_controls.get(&key) {
                Some(figure_control_signal.clone())
            } else {
                figure_controls.insert(
                    key,
                    create_static_signal(scope, figure_control_data.clone()),
                );
                None
            }
        };
        opt_figure_control_signal.map(|signal| signal.set(figure_control_data));
    }

    pub(crate) fn figure_control_data(
        &self,
        trace: &TraceData,
        presentation: &Presentation,
    ) -> &'static Signal<FigureControlData> {
        self.figure_controls.borrow(file!(), line!())
            [&FigureControlKey::new(trace.opt_parent_id, trace.kind, trace.id, presentation)]
    }

    pub(crate) fn did_toggle_pin(&self, trace_id: TraceId) {
        let mut presentation = self.presentation_signal.cget();
        presentation.toggle_pin(trace_id);
        self.presentation_signal.set(presentation)
    }
}

impl DeveloperGuiContext {
    pub(crate) fn figure_canvas_value(&self) -> FigureCanvasValue {
        todo!()
    }
    // pub(crate) fn collect_pinned_canvas_values(&'static self) -> Vec<FigureCanvasDataItd> {
    //     let restriction = self.presentation_signal().get();
    //     restriction
    //         .pins()
    //         .iter()
    //         .map(|pin| {
    //             let specific_key =
    //                 FigureCanvasKey::from_trace_data(self.trace_data(*pin), &restriction, true);
    //             let generic_key =
    //                 FigureCanvasKey::from_trace_data(self.trace_data(*pin), &restriction, false);
    //             let generic_value = self.figure_canvases.borrow(file!(), line!())[&generic_key];
    //             let specific_value = self.figure_canvases.borrow(file!(), line!())[&specific_key];
    //             match specific_value {
    //                 FigureCanvasData::Plot2d {
    //                     plot_kind,
    //                     point_groups,
    //                     xrange,
    //                     yrange,
    //                 } => todo!(),
    //                 FigureCanvasData::Mutations { mutations } => todo!(),
    //                 FigureCanvasData::GenericGraphics2d {
    //                     partitioned_samples,
    //                 } => todo!(),
    //                 FigureCanvasData::GenericF32 {
    //                     partitioned_samples,
    //                 } => todo!(),
    //                 FigureCanvasData::GenericI32 {
    //                     partitioned_samples,
    //                 } => todo!(),
    //                 FigureCanvasData::EvalError { message } => todo!(),
    //                 _ => (),
    //             }
    //             FigureCanvasDataItd {
    //                 generic: generic_value,
    //                 specific: specific_value,
    //             }
    //         })
    //         .collect()
    // }
}
