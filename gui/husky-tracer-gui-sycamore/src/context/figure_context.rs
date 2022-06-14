use super::*;

#[derive(Debug, Default)]
pub struct FigureContext {
    figures: RefCell<HashMap<FigureCanvasKey, Rc<FigureCanvasData>>>,
    figure_control_stores: RefCell<HashMap<FigureControlKey, Signal<FigureControlData>>>,
}

impl FigureContext {
    pub(super) fn init(
        &self,
        figures: Vec<(FigureCanvasKey, FigureCanvasData)>,
        figure_controls: Vec<(FigureControlKey, FigureControlData)>,
    ) {
        *self.figures.borrow_mut() = figures.into_iter().map(|(k, v)| (k, Rc::new(v))).collect();
        *self.figure_control_stores.borrow_mut() = figure_controls
            .into_iter()
            .map(|(k, v)| (k, Signal::new(v)))
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
            .figures
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
        self.figures.borrow()[&FigureCanvasKey::new(trace, focus)].clone()
    }

    pub(super) fn is_figure_cached(&self, trace: &TraceData, focus: &Focus) -> bool {
        let key = FigureCanvasKey::new(trace, focus);
        self.figures.borrow().contains_key(&key)
    }

    fn set_figure_control_data(
        &self,
        trace: &TraceData,
        focus: &Focus,
        figure_control_data: FigureControlData,
    ) {
        let figure_control_stores = &mut self.figure_control_stores.borrow_mut();
        let key = FigureControlKey::new(trace, focus);
        if let Some(figure_control_signal) = figure_control_stores.get(&key) {
            figure_control_signal.set(figure_control_data)
        } else {
            figure_control_stores.insert(key, Signal::new(figure_control_data));
        }
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
