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
    pub(crate) fn figure_canvas_value_signal<'a>(
        &'static self,
        scope: Scope<'a>,
    ) -> &'a ReadSignal<FigureCanvasValue> {
        memo!(scope, move || {
            let presentation = &self.presentation_signal.get();
            let opt_active_figure_not_pinned = presentation
                .opt_active_trace_id()
                .map(|trace_id| {
                    if presentation.is_pinned(trace_id) {
                        None
                    } else {
                        Some(self.figure_canvas_data_itd(trace_id, presentation))
                    }
                })
                .flatten();
            FigureCanvasValue::new(
                presentation.kind(),
                opt_active_figure_not_pinned,
                self.figure_canvas_data_itds(presentation),
            )
        })
    }

    fn figure_canvas_data_itds(
        &'static self,
        presentation: &Presentation,
    ) -> Vec<FigureCanvasDataItd> {
        presentation
            .pins()
            .iter()
            .map(|pin| self.figure_canvas_data_itd(*pin, presentation))
            .collect()
    }

    fn figure_canvas_data_itd(
        &'static self,
        trace_id: TraceId,
        presentation: &Presentation,
    ) -> FigureCanvasDataItd {
        let specific_key =
            FigureCanvasKey::from_trace_data(self.trace_data(trace_id), presentation, true);
        let generic_key =
            FigureCanvasKey::from_trace_data(self.trace_data(trace_id), presentation, false);
        let generic_value = self.figure_canvases.borrow(file!(), line!())[&generic_key];
        // ad hoc
        let generic_value = generic_value
            .generic()
            .unwrap_or(&GenericFigureCanvasData::Unit);
        let specific_value = self.figure_canvases.borrow(file!(), line!())[&specific_key];
        log::info!("specific_value = {specific_value:?}");
        let specific_value = specific_value.specific().unwrap();
        match specific_value {
            SpecificFigureCanvasData::Mutations { mutations } => todo!(),
            SpecificFigureCanvasData::EvalError { message } => todo!(),
            _ => (),
        }
        FigureCanvasDataItd {
            generic: generic_value,
            specific: specific_value,
        }
    }
}
