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
    pub fn id(&mut self, trace: Trace) -> TraceId {
        if let Some(&id) = self.ids.get(&trace) {
            return id;
        }
        self.alloc_new(trace)
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
