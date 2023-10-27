use crate::*;
use std::collections::HashMap;

pub struct TraceIdMap<Trace>
where
    Trace: Eq + std::hash::Hash + Copy,
{
    traces: Vec<Trace>,
    ids: HashMap<Trace, TraceId>,
}

impl<Trace> Default for TraceIdMap<Trace>
where
    Trace: Eq + std::hash::Hash + Copy,
{
    fn default() -> Self {
        Self {
            traces: Default::default(),
            ids: Default::default(),
        }
    }
}

impl<Trace> TraceIdMap<Trace>
where
    Trace: Eq + std::hash::Hash + Copy,
{
    pub fn trace_id_or_alloc_new(
        &mut self,
        trace: Trace,
        alloc_effect: impl FnOnce(TraceId),
    ) -> TraceId {
        if let Some(&id) = self.ids.get(&trace) {
            return id;
        }
        let id = self.alloc_new(trace);
        alloc_effect(id);
        id
    }

    pub fn trace(&mut self, trace_id: TraceId) -> Trace {
        self.traces[trace_id.index()]
    }

    fn alloc_new(&mut self, trace: Trace) -> TraceId {
        debug_assert_eq!(self.traces.len(), self.ids.len());
        let raw = self.traces.len();
        self.traces.push(trace);
        let id = TraceId::from_index(raw);
        self.ids.insert(trace, id);
        id
    }
}
