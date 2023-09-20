use vec_like::VecSet;

use super::*;

impl DeveloperGuiContext {
    // pub(super) fn receive_figure_canvases(
    //     &self,
    //     visibility: Scope,
    //     new_figure_canvases: impl Iterator<Item = (FigureCanvasKey, &'static FigureCanvasData)>,
    // ) {
    //     let mut figure_canvases = self.figure_canvases.borrow_mut(file!(), line!());
    //     for (key, data) in new_figure_canvases {
    //         insert_new!(figure_canvases, key, data);
    //     }
    // }
    // pub(super) fn receive_figure_controls(
    //     &self,
    //     visibility: Scope,
    //     new_figure_controls: impl Iterator<Item = (FigureControlKey, FigureControlData)>,
    // ) {
    //     let mut figure_controls = self.figure_controls.borrow_mut(file!(), line!());
    //     for (key, data) in new_figure_controls {
    //         assert!(figure_controls
    //             .insert(key, create_signal(visibility, data))
    //             .is_none());
    //     }
    // }

    // pub(crate) fn new_figure_canvas_key(
    //     &self,
    //     trace: &TraceData,
    //     presentation: &Presentation,
    //     is_specific: bool,
    // ) -> FigureCanvasKey {
    //     FigureCanvasKey::new(trace.kind, trace.id, presentation, is_specific)
    // }

    fn figure_canvas_data(&self) -> FigureCanvasData<'a> {
        FigureCanvasData<'a> {
            generic: todo!(),
            specific: todo!(),
        }
    }

    fn set_figure_control_data(
        &self,
        visibility: Scope,
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
                    create_static_signal(visibility, figure_control_data.clone()),
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

    pub(crate) fn figure_canvas_data_itds(
        &'static self,
        presentation: &Presentation,
    ) -> Vec<FigureCanvasData<'a>> {
        presentation
            .pins()
            .iter()
            .map(|pin| self.figure_canvas_data_itd(*pin, presentation))
            .collect()
    }

    pub(crate) fn figure_canvas_data_itd(
        &'static self,
        trace_id: TraceId,
        presentation: &Presentation,
    ) -> FigureCanvasData<'a> {
        let trace_data = &self.trace_data(trace_id);
        FigureCanvasData<'a> {
            generic: self.generic_figure_canvas_data(trace_data, presentation),
            specific: self.specific_figure_canvas_data(trace_data, presentation),
        }
    }

    fn generic_figure_canvas_data(
        &self,
        trace: &TraceData,
        presentation: &Presentation,
    ) -> &'static GenericFigureCanvasData {
        let key = match GenericFigureCanvasKey::from_trace_data(trace, presentation) {
            Some(key) => key,
            None => return &GenericFigureCanvasData::Unit,
        };
        let figure_canvases_borrowed = self.generic_figure_canvases.borrow(file!(), line!());
        if let Some(figure_canvas_data) = figure_canvases_borrowed.get(&key) {
            figure_canvas_data
        } else {
            // ad hoc
            log::info!("presentation = {presentation:?}");
            log::info!("generic figure canvases: {figure_canvases_borrowed:?}");
            log::info!("no entry with key {key:?}");
            panic!()
        }
    }

    fn specific_figure_canvas_data(
        &self,
        trace: &TraceData,
        presentation: &Presentation,
    ) -> &'static SpecificFigureCanvasData {
        let key = match SpecificFigureCanvasKey::from_trace_data(trace, presentation) {
            Some(key) => key,
            None => return &SpecificFigureCanvasData::Unit,
        };
        let figure_canvases_borrowed = self.specific_figure_canvases.borrow(file!(), line!());
        if let Some(figure_canvas_data) = figure_canvases_borrowed.get(&key) {
            figure_canvas_data
        } else {
            // ad hoc
            log::info!("presentation = {presentation:?}");
            log::info!("specific figure canvases: {figure_canvases_borrowed:?}");
            log::info!("no entry with key {key:?}");
            panic!()
        }
    }
}
