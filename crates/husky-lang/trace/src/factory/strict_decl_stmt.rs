use semantics::DeclStmt;
use text::Text;
use vm::{History, VMControl};

use super::*;
use crate::*;

impl TraceFactory {
    pub fn new_strict_decl_stmt_trace(
        &self,
        parent_id: TraceId,
        indent: Indent,
        stmt: Arc<DeclStmt>,
        history: Arc<History>,
        text: &Text,
    ) -> Arc<Trace> {
        self.new_trace(
            Some(parent_id),
            indent,
            TraceKind::StrictDeclStmt { stmt, history },
            text,
        )
    }
}
