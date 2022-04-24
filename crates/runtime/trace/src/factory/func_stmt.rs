use text::Text;
use vm::{History, VMControl};

use super::*;
use crate::*;

impl<'eval> TraceFactory<'eval> {
    pub fn new_func_stmt_trace(
        &self,
        parent_id: TraceId,
        indent: Indent,
        stmt: Arc<FuncStmt>,
        history: Arc<History<'eval>>,
        text: &Text,
    ) -> Arc<Trace<'eval>> {
        self.new_trace(
            Some(parent_id),
            indent,
            TraceVariant::FuncStmt { stmt, history },
            text,
        )
    }
    pub fn func_stmts_traces<'a>(
        &'a self,
        parent_id: TraceId,
        indent: Indent,
        stmts: &'a [Arc<FuncStmt>],
        text: &'a Text,
        history: &'a Arc<History<'eval>>,
    ) -> impl Iterator<Item = Arc<Trace<'eval>>> + 'a {
        stmts.iter().map(move |stmt| {
            self.new_func_stmt_trace(parent_id, indent, stmt.clone(), history.clone(), text)
        })
    }
}
