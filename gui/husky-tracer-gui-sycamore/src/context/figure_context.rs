use super::*;

#[derive(Debug, Default)]
pub struct FigureContext {
    figures: RefCell<HashMap<FigureKey, Rc<FigureProps>>>,
    figure_control_stores: RefCell<HashMap<FigureControlKey, Signal<FigureControlProps>>>,
}

impl FigureContext {
    pub(super) fn init(
        &self,
        figures: HashMap<FigureKey, FigureProps>,
        figure_controls: HashMap<FigureControlKey, FigureControlProps>,
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
        figure: Rc<FigureProps>,
        figure_control_props: FigureControlProps,
    ) {
        todo!()
        // assert!(self.figures.insert((trace.id, focus), figure).is_none());
        // self.set_figure_control_props(trace, figure_control_props);
    }

    pub(super) fn get_figure(&mut self, trace_id: TraceId, focus: Focus) -> Rc<FigureProps> {
        todo!()
        //    self.figures[(trace_id, focus)]
    }

    fn set_figure_control_props(
        &mut self,
        trace: &TraceData,
        figure_control_props: Rc<FigureControlProps>,
    ) {
        todo!()
        // self.figure_control_stores[key]=figure_control_props);
    }

    // fn update_figure_control_props(
    //     &mut self,
    //     trace_id: TraceId,
    //     updater: Updater<FigureControlProps>
    // ) {
    //     let key = self.figure_control_key(trace);
    //     self.figure_control_stores.update(key, updater);
    // }

    // get_figure_control_store(trace: Trace): Readable<FigureControlProps> {
    //     let key = self.figure_control_key(trace);
    //     return self.figure_control_stores.get_store(key);
    // }
}
