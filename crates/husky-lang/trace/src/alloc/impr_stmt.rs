use semantics::ImprStmt;
use text::Text;

use super::*;
use crate::*;

impl TraceAllocator {
    pub fn new_impr_stmt_trace(
        &self,
        parent: TraceId,
        indent: Indent,
        stmt: Arc<ImprStmt>,
        exec: impl FnOnce(TraceId) -> (TraceInterpreterControlSignal, Vec<TokenProps>),
        text: &Text,
    ) -> Arc<Trace> {
        self.new_trace2(
            parent,
            indent,
            |trace_id| {
                let (control_signal, tokens) = exec(trace_id);
                TraceKind::ImprStmt {
                    stmt,
                    tokens,
                    control_signal,
                }
            },
            text,
        )
    }
}
