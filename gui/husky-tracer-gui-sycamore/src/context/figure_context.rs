use super::*;

#[derive(Debug, Default)]
pub struct FigureContext {
    figure_canvases: RefCell<HashMap<FigureCanvasKey, Rc<FigureCanvasData>>>,
    figure_controls: RefCell<HashMap<FigureControlKey, Rc<Signal<FigureControlData>>>>,
}

impl FigureContext {
    pub(super) fn init(
        &self,
        figure_canvases: Vec<(FigureCanvasKey, FigureCanvasData)>,
        figure_controls: Vec<(FigureControlKey, FigureControlData)>,
    ) {
        *self.figure_canvases.borrow_mut() = figure_canvases
            .into_iter()
            .map(|(k, v)| (k, Rc::new(v)))
            .collect();
        *self.figure_controls.borrow_mut() = figure_controls
            .into_iter()
            .map(|(k, v)| (k, Rc::new(Signal::new(v))))
            .collect();
    }

    pub(super) fn set_figure(
        &self,
        trace: &TraceData,
        focus: &Focus,
        figure: FigureCanvasData,
        figure_control_props: FigureControlData,
    ) {
        assert!(self
            .figure_canvases
            .borrow_mut()
            .insert(FigureCanvasKey::new(trace, focus), Rc::new(figure))
            .is_none());
        self.set_figure_control_data(trace, focus, figure_control_props);
    }

    pub(crate) fn figure_canvas_data(
        &self,
        trace: &TraceData,
        focus: &Focus,
    ) -> Rc<FigureCanvasData> {
        let figure_canvas_key = FigureCanvasKey::new(trace, focus);
        self.figure_canvases.borrow()[&figure_canvas_key].clone()
    }

    pub(super) fn is_figure_cached(&self, trace: &TraceData, focus: &Focus) -> bool {
        let key = FigureCanvasKey::new(trace, focus);
        self.figure_canvases.borrow().contains_key(&key)
    }

    fn set_figure_control_data(
        &self,
        trace: &TraceData,
        focus: &Focus,
        figure_control_data: FigureControlData,
    ) {
        let figure_controls = &mut self.figure_controls.borrow_mut();
        let key = FigureControlKey::new(trace, focus);
        if let Some(figure_control_signal) = figure_controls.get(&key) {
            figure_control_signal.set(figure_control_data)
        } else {
            figure_controls.insert(key, Rc::new(Signal::new(figure_control_data)));
        }
    }

    pub(crate) fn figure_control_data(
        &self,
        trace: &TraceData,
        focus: &Focus,
    ) -> Rc<Signal<FigureControlData>> {
        self.figure_controls.borrow()[&FigureControlKey::new(trace, focus)].clone()
    }

    // fn update_figure_control_props(
    //     &mut self,
    //     trace_id: TraceId,
    //     updater: Updater<FigureControlData>
    // ) {
    //     let key = self.figure_control_key(trace);
    //     self.figure_control_stores.update(key, updater);
    // }

    // get_figure_control_store(trace: Trace): Readable<FigureControlData> {
    //     let key = self.figure_control_key(trace);
    //     return self.figure_control_stores.get_store(key);
    // }
}
