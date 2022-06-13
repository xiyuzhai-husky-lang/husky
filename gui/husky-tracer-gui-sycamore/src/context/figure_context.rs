use super::*;

#[derive(Debug, Default)]
pub struct FigureContext {
    figures: RefCell<HashMap<FigureKey, Rc<FigureContentData>>>,
    figure_control_stores: RefCell<HashMap<FigureControlKey, Signal<FigureControlData>>>,
}

impl FigureContext {
    pub(super) fn init(
        &self,
        figures: Vec<(FigureKey, FigureContentData)>,
        figure_controls: Vec<(FigureControlKey, FigureControlData)>,
    ) {
        *self.figures.borrow_mut() = figures.into_iter().map(|(k, v)| (k, Rc::new(v))).collect();
        *self.figure_control_stores.borrow_mut() = figure_controls
            .into_iter()
            .map(|(k, v)| (k, Signal::new(v)))
            .collect();
    }

    pub(super) fn set_figure(
        &mut self,
        trace: &TraceData,
        focus: Focus,
        figure: Rc<FigureContentData>,
        figure_control_props: FigureControlData,
    ) {
        todo!()
        // assert!(self.figures.insert((trace.id, focus), figure).is_none());
        // self.set_figure_control_props(trace, figure_control_props);
    }

    pub(super) fn get_figure(&mut self, trace_id: TraceId, focus: Focus) -> Rc<FigureContentData> {
        todo!()
        //    self.figures[(trace_id, focus)]
    }

    fn set_figure_control_props(
        &mut self,
        trace: &TraceData,
        figure_control_props: Rc<FigureControlData>,
    ) {
        todo!()
        // self.figure_control_stores[key]=figure_control_props);
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
