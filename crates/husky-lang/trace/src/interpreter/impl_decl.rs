use super::*;

impl TraceInterpreter {
    pub fn decl_stmt_traces(
        &self,
        parent: &Trace,
        stmts: &[Arc<DeclStmt>],
        indent: Indent,
    ) -> Vec<Arc<Trace>> {
        let mut traces = vec![];
        for stmt in stmts {
            let trace = self.trace_allocator.new_strict_decl_stmt_trace(
                parent.id,
                indent,
                stmt.clone(),
                |trace_id| self.exec_decl_stmt(trace_id, indent, stmt),
                &self.text,
            );
            let stop = match trace.kind {
                TraceKind::StrictDeclStmt {
                    ref control_signal, ..
                } => match control_signal {
                    TraceInterpreterControlSignal::Return(_)
                    | TraceInterpreterControlSignal::Err(_) => true,
                    TraceInterpreterControlSignal::None => false,
                },
                _ => panic!(),
            };
            traces.push(trace);
            if stop {
                break;
            }
        }
        traces
    }

    fn exec_decl_stmt(
        &self,
        trace_id: TraceId,
        indent: Indent,
        stmt: &DeclStmt,
    ) -> (TraceInterpreterControlSignal, Vec<TokenProps>) {
        match stmt.kind {
            DeclStmtKind::Init { varname, ref value } => todo!(),
            DeclStmtKind::Assert { ref condition } => todo!(),
            DeclStmtKind::Return { ref result } => {
                let (result, tokens) = self.exec_expr(trace_id, indent + 4, result.clone(), true);
                let control_signal = match result {
                    Ok(value) => TraceInterpreterControlSignal::Return(value),
                    Err(error) => TraceInterpreterControlSignal::Err(error),
                };
                (control_signal, tokens)
            }
            DeclStmtKind::Branches { kind, ref branches } => todo!(),
        }
    }
}
