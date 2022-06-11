use super::*;

#[derive(Debug, Default)]
pub struct FocusContext {
    focus_store: Signal<Focus>,
    focus_locked_store: Signal<bool>,
}
impl FocusContext {
    pub(super) fn init(&mut self, focus: Focus) {
        self.focus_locked_store.set(true);
        self.focus_store.set(focus);
    }

    pub(super) fn opt_input_id(&self) -> Option<usize> {
        return self.focus_store.get().opt_input_id;
    }

    pub(super) fn did_lock_focus(&mut self, focus: Focus) {
        self.focus_store.set(focus);
        self.focus_locked_store.set(true);
    }

    pub(super) fn focus(&self) -> Focus {
        return self.focus_store.get_cloned();
    }
}
