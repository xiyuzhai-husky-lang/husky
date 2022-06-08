use std::rc::Rc;
#[derive(PartialEq, Eq)]
pub struct DebuggerState {}

impl DebuggerState {
    pub fn new() -> Rc<DebuggerState> {
        Rc::new(DebuggerState {})
    }
}
