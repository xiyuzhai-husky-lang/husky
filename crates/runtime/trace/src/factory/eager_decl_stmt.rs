use text::Text;
use vm::{History, VMControl};

use super::*;
use crate::*;

impl<'eval> TraceFactory<'eval> {
    pub fn new_strict_decl_stmt_trace(
        &self,
        parent_id: TraceId,
        indent: Indent,
        stmt: Arc<DeclStmt>,
        history: Arc<History<'eval>>,
        text: &Text,
    ) -> Arc<Trace<'eval>> {
        self.new_trace(
            Some(parent_id),
            indent,
            TraceKind::StrictDeclStmt { stmt, history },
            text,
        )
    }
}
