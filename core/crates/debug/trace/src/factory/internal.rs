use super::*;

#[derive(Debug)]
pub struct TraceNode {
    pub(super) trace: Arc<Trace>,
    pub(super) expansion: bool,
    pub(super) shown: bool,
}

impl TraceFactoryInternal {
    pub(super) fn clear(&mut self) {
        self.traces.clear();
        self.expansions.clear();
        self.showns.clear();
    }

    pub(super) fn toggle_expansion(&mut self, id: TraceId) {
        let expanded = self.expansions.entry(id).or_insert(false);
        *expanded = !*expanded;
    }

    pub(super) fn is_expanded(&mut self, trace: &Trace) -> bool {
        *self.expansions.entry(trace.id()).or_insert(false)
    }

    pub(super) fn toggle_show(&mut self, id: TraceId) {
        let shown = self.showns.entry(id).or_insert(false);
        *shown = !*shown;
    }
}
