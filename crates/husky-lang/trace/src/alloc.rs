mod feature_block;
mod feature_branch;
mod feature_expr;
mod feature_stmt;

use stdx::sync::ARwLock;

use crate::*;

#[derive(Default)]
pub struct TraceAllocator {
    traces: ARwLock<HashMap<usize, Arc<Trace>>>,
}

impl TraceAllocator {
    pub fn new_trace(&self, parent: Option<usize>, indent: Indent, kind: TraceKind) -> Arc<Trace> {
        let trace = Arc::new(Trace::new(parent, indent, kind));
        self.traces
            .write(|traces| traces.insert(trace.id, trace.clone()));
        trace
    }

    fn trace(&self, id: usize) -> Arc<Trace> {
        self.traces.read(|traces| traces.get(&id).unwrap().clone())
    }

    pub fn clear(&self) {
        self.traces.write(|traces| traces.clear())
    }
}

pub trait AllocateTrace {
    fn trace_allocator(&self) -> &TraceAllocator;

    fn new_trace(&self, parent: Option<usize>, indent: Indent, kind: TraceKind) -> Arc<Trace> {
        self.trace_allocator().new_trace(parent, indent, kind)
    }

    fn trace(&self, id: usize) -> Arc<Trace> {
        self.trace_allocator().trace(id)
    }

    fn clear(&self) {
        self.trace_allocator().clear()
    }
}
