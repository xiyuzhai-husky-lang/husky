use vec_like::VecSet;

use super::*;

#[derive(Debug)]
pub struct FigureContext {
    pub(crate) figure_canvases: RefCell<HashMap<FigureCanvasKey, &'static FigureCanvasData>>,
    pub(crate) figure_controls:
        RefCell<HashMap<FigureControlKey, &'static Signal<FigureControlData>>>,
    pins: &'static Signal<VecSet<TraceId>>,
    arrivals: &'static Signal<VecSet<TraceId>>,
    enters: &'static Signal<VecSet<TraceId>>,
}
impl FigureContext {
    pub(super) fn new<'a>(scope: Scope<'a>) -> Self {
        Self {
            figure_canvases: Default::default(),
            figure_controls: Default::default(),
            pins: create_static_signal(scope, Default::default()),
            arrivals: create_static_signal(scope, Default::default()),
            enters: create_static_signal(scope, Default::default()),
        }
    }

    pub(super) fn init(
        &self,
        figure_canvases: HashMap<FigureCanvasKey, &'static FigureCanvasData>,
        figure_controls: HashMap<FigureControlKey, &'static Signal<FigureControlData>>,
    ) {
        *self.figure_canvases.borrow_mut(file!(), line!()) = figure_canvases;
        *self.figure_controls.borrow_mut(file!(), line!()) = figure_controls;
    }

    pub(super) fn set_opt_figure_data(
        &self,
        scope: Scope<'static>,
        trace: &TraceData,
        attention: &Attention,
        opt_figure_canvas_data: Option<&'static FigureCanvasData>,
        figure_control_data: Option<FigureControlData>,
    ) {
        if let Some(figure_canvas_data) = opt_figure_canvas_data {
            assert!(self
                .figure_canvases
                .borrow_mut(file!(), line!())
                .insert(
                    self.new_figure_canvas_key(trace, attention,),
                    figure_canvas_data
                )
                .is_none());
        }
        if let Some(figure_control_data) = figure_control_data {
            self.set_figure_control_data(scope, trace, attention, figure_control_data);
        }
    }

    pub(crate) fn new_figure_canvas_key(
        &self,
        trace: &TraceData,
        attention: &Attention,
    ) -> FigureCanvasKey {
        FigureCanvasKey::new(
            trace.kind,
            trace.id,
            attention,
            self.enters.get().to_vec(),
            self.arrivals.get().to_vec(),
            self.pins.get().to_vec(),
        )
    }

    pub(crate) fn figure_canvas_data(
        &self,
        trace: &TraceData,
        attention: &Attention,
    ) -> &'static FigureCanvasData {
        let figure_canvas_key = self.new_figure_canvas_key(trace, attention);
        let figure_canvases_borrowed = self.figure_canvases.borrow(file!(), line!());
        if let Some(figure_canvas_data) = figure_canvases_borrowed.get(&figure_canvas_key) {
            figure_canvas_data
        } else {
            log::info!("no entry with key {figure_canvas_key:?}");
            panic!()
        }
    }

    fn set_figure_control_data(
        &self,
        scope: Scope<'static>,
        trace: &TraceData,
        attention: &Attention,
        figure_control_data: FigureControlData,
    ) {
        let opt_figure_control_signal = {
            let figure_controls = &mut self.figure_controls.borrow_mut(file!(), line!());
            let key = FigureControlKey::new(trace.opt_parent_id, trace.kind, trace.id, attention);
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
        attention: &Attention,
    ) -> &'static Signal<FigureControlData> {
        self.figure_controls.borrow(file!(), line!())
            [&FigureControlKey::new(trace.opt_parent_id, trace.kind, trace.id, attention)]
    }

    pub(crate) fn did_toggle_arrival(&self, trace_id: TraceId) {
        todo!()
    }
}
