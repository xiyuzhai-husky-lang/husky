use semantics::ImprStmtKind;

use super::*;

impl TraceInterpreter {
    pub fn impr_stmt_traces(
        mut self,
        parent: &Trace,
        stmts: &[Arc<ImprStmt>],
        indent: Indent,
    ) -> Vec<Arc<Trace>> {
        let mut traces = vec![];
        let trace_allocator = self.trace_allocator.clone();
        let text = self.text.clone();
        for stmt in stmts {
            let trace = trace_allocator.new_impr_stmt_trace(
                parent.id,
                indent,
                stmt.clone(),
                |trace_id| self.exec_impr_stmt(trace_id, indent, stmt),
                &text,
            );
            let stop = match trace.kind {
                TraceKind::ImprStmt {
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

    fn exec_impr_stmt(
        &mut self,
        trace_id: TraceId,
        indent: Indent,
        stmt: &ImprStmt,
    ) -> (TraceInterpreterControlSignal, Vec<TokenProps>) {
        match stmt.kind {
            ImprStmtKind::Init {
                varname,
                ref initial_value,
                init_kind,
                varidx,
            } => {
                let mut tokens = vec![init_kind.into(), ident!(varname.0)];
                let (initial_value_result, expr_tokens) =
                    self.exec_expr(trace_id, indent + 4, initial_value.clone(), true);
                tokens.push(special!(" = "));
                tokens.extend(expr_tokens);
                let control_signal = match initial_value_result {
                    Ok(value) => {
                        self.init_trace_stack_value(varidx, value);
                        TraceInterpreterControlSignal::None
                    }
                    Err(error) => TraceInterpreterControlSignal::Err(error),
                };
                (control_signal, tokens)
            }
            ImprStmtKind::Assert { ref condition } => todo!(),
            ImprStmtKind::Return { ref result } => {
                let mut tokens = vec![keyword!("return ")];
                let (result, expr_tokens) =
                    self.exec_expr(trace_id, indent + 4, result.clone(), true);
                tokens.extend(expr_tokens);
                let control_signal = match result {
                    Ok(value) => TraceInterpreterControlSignal::Return(value),
                    Err(error) => TraceInterpreterControlSignal::Err(error),
                };
                (control_signal, tokens)
            }
            ImprStmtKind::BranchGroup { kind, ref branches } => todo!(),
            ImprStmtKind::Loop => todo!(),
        }
    }
}
