use super::*;
pub struct TracerContextInternal {
    pub(super) ws: WebsocketService,
    pub(super) call_backs: HashMap<usize, Box<dyn FnOnce(&mut Self, DebuggerServerMessage)>>,
    pub tree_context: TreeContext,
    pub figure_context: FigureContext,
    pub focus_context: FocusContext,
    pub signal: Signal<i32>,
}

impl TracerContextInternal {
    pub fn new(ws: WebsocketService) -> TracerContextInternal {
        TracerContextInternal {
            signal: Signal::new(0),
            ws,
            call_backs: Default::default(),
            tree_context: Default::default(),
            figure_context: Default::default(),
            focus_context: Default::default(),
        }
    }
}
