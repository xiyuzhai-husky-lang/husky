use vec_like::VecSet;

use super::*;

#[derive(Debug)]
pub struct FigureContext {
    pub(crate) figure_canvases: RefCell<HashMap<FigureCanvasKey, &'static FigureCanvasData>>,
    pub(crate) figure_controls:
        RefCell<HashMap<FigureControlKey, &'static Signal<FigureControlData>>>,
    pub(crate) pins: &'static Signal<VecSet<TraceId>>,
}

impl FigureContext {
    pub(super) fn new<'a>(scope: Scope<'a>) -> Self {
        Self {
            figure_canvases: Default::default(),
            figure_controls: Default::default(),
            pins: create_static_signal(scope, Default::default()),
        }
    }

    pub(super) fn init(
        &self,
        figure_canvases: HashMap<FigureCanvasKey, &'static FigureCanvasData>,
        figure_controls: HashMap<FigureControlKey, &'static Signal<FigureControlData>>,
        pins: VecSet<TraceId>,
    ) {
        *self.figure_canvases.borrow_mut(file!(), line!()) = figure_canvases;
        *self.figure_controls.borrow_mut(file!(), line!()) = figure_controls;
        self.pins.set(pins)
    }

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
        restriction: &Restriction,
    ) -> FigureCanvasKey {
        FigureCanvasKey::new(trace.kind, trace.id, restriction)
    }

    pub(crate) fn figure_canvas_data(
        &self,
        trace: &TraceData,
        restriction: &Restriction,
    ) -> &'static FigureCanvasData {
        let figure_canvas_key = self.new_figure_canvas_key(trace, restriction);
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
        restriction: &Restriction,
    ) -> &'static Signal<FigureControlData> {
        self.figure_controls.borrow(file!(), line!())
            [&FigureControlKey::new(trace.opt_parent_id, trace.kind, trace.id, restriction)]
    }

    pub(crate) fn did_toggle_pin(&self, trace_id: TraceId) {
        let mut new_pins = self.pins.cget();
        new_pins.toggle(trace_id);
        self.pins.set(new_pins);
    }
}
