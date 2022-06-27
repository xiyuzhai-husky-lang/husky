use semantics_entity::{EntityDefn, EntityDefnVariant};
use word::Identifier;

use super::*;

impl HuskyTraceTime {
    pub(crate) fn feature_stmt_traces(
        &mut self,
        parent: &Trace,
        stmt: Arc<FeatureStmt>,
    ) -> Vec<TraceId> {
        match stmt.variant {
            FeatureLazyStmtVariant::Init { .. }
            | FeatureLazyStmtVariant::Assert { .. }
            | FeatureLazyStmtVariant::Return { .. } => {
                vec![self.new_trace(
                    Some(parent.id()),
                    stmt.indent,
                    TraceVariant::FeatureLazyStmt(stmt),
                )]
            }
            FeatureLazyStmtVariant::ConditionFlow { ref branches, .. } => branches
                .iter()
                .map(|branch| self.feature_branch_trace(parent, stmt.indent, branch.clone()))
                .collect(),
            FeatureLazyStmtVariant::ReturnXml { ref result } => todo!(),
        }
    }

    pub(crate) fn feature_stmt_lines(&mut self, stmt: &FeatureStmt) -> Vec<TraceLineData> {
        vec![TraceLineData {
            indent: stmt.indent,
            idx: 0,
            tokens: self.feature_stmt_tokens(stmt),
        }]
    }

    pub(crate) fn feature_branch_trace(
        &mut self,
        parent: &Trace,
        indent: Indent,
        branch: Arc<FeatureLazyBranch>,
    ) -> TraceId {
        self.new_trace(
            Some(parent.id()),
            indent,
            TraceVariant::FeatureLazyBranch(branch),
        )
    }

    pub(crate) fn func_stmts_traces<'a>(
        &'a mut self,
        parent_id: TraceId,
        indent: Indent,
        stmts: &'a [Arc<FuncStmt>],
        history: &'a Arc<History<'static>>,
    ) -> impl Iterator<Item = TraceId> + 'a {
        stmts.iter().map(move |stmt| {
            self.new_func_stmt_trace(parent_id, indent, stmt.clone(), history.clone())
        })
    }

    pub(crate) fn new_func_stmt_trace(
        &mut self,
        parent_id: TraceId,
        indent: Indent,
        stmt: Arc<FuncStmt>,
        history: Arc<History<'static>>,
    ) -> TraceId {
        self.new_trace(
            Some(parent_id),
            indent,
            TraceVariant::FuncStmt { stmt, history },
        )
    }
    pub(crate) fn new_proc_stmt_trace(
        &mut self,
        parent_id: TraceId,
        indent: Indent,
        stmt: Arc<ProcStmt>,
        history: Arc<History<'static>>,
    ) -> TraceId {
        self.new_trace(
            Some(parent_id),
            indent,
            TraceVariant::ProcStmt { stmt, history },
        )
    }

    pub(crate) fn new_proc_branch_trace(
        &mut self,
        parent_id: TraceId,
        indent: Indent,
        stmt: Arc<ProcStmt>,
        branch: Arc<ProcConditionBranch>,
        branch_idx: u8,
        history: Arc<History<'static>>,
    ) -> TraceId {
        let opt_vm_branch = history.get(&stmt).map(|entry| match entry {
            HistoryEntry::ControlFlow { vm_branches, .. } => {
                vm_branches[branch_idx as usize].clone()
            }
            _ => panic!(),
        });
        self.new_trace(
            Some(parent_id),
            indent,
            TraceVariant::ProcBranch {
                stmt,
                branch,
                branch_idx,
                opt_vm_branch,
                history,
            },
        )
    }

    pub(crate) fn proc_stmts_traces(
        &mut self,
        parent_id: TraceId,
        indent: Indent,
        stmts: &[Arc<ProcStmt>],
        history: &Arc<History<'static>>,
    ) -> Vec<TraceId> {
        let mut traces = Vec::new();
        for stmt in stmts {
            match stmt.variant {
                ProcStmtVariant::ConditionFlow { ref branches } => {
                    for (branch_idx, branch) in branches.iter().enumerate() {
                        traces.push(self.new_proc_branch_trace(
                            parent_id,
                            indent,
                            stmt.clone(),
                            branch.clone(),
                            branch_idx.try_into().unwrap(),
                            history.clone(),
                        ))
                    }
                }
                _ => traces.push(self.new_proc_stmt_trace(
                    parent_id,
                    indent,
                    stmt.clone(),
                    history.clone(),
                )),
            }
        }
        traces
    }

    pub(crate) fn new_eager_expr_trace(
        &mut self,
        expr: Arc<EagerExpr>,
        history: Arc<History<'static>>,
        opt_parent: Option<&Trace>,
        indent: Indent,
    ) -> TraceId {
        self.new_trace(
            opt_parent.map(|parent| parent.id()),
            indent,
            TraceVariant::EagerExpr { expr, history },
        )
    }

    pub(crate) fn new_call_head_trace(
        &mut self,
        parent: &Trace,
        entity: Arc<EntityDefn>,
    ) -> TraceId {
        let tokens = match entity.variant {
            EntityDefnVariant::Func { ref parameters, .. } => routine_call_head_tokens(
                &self
                    .eval_time_singleton
                    .compile_time()
                    .text(entity.file)
                    .unwrap(),
                "func ",
                entity.ident,
                parameters,
            ),
            EntityDefnVariant::Proc {
                parameters: ref parameters,
                ..
            } => routine_call_head_tokens(
                &self
                    .eval_time_singleton
                    .compile_time()
                    .text(entity.file)
                    .unwrap(),
                "proc ",
                entity.ident,
                parameters,
            ),
            _ => todo!(),
        };
        return self.new_trace(
            Some(parent.id()),
            0,
            TraceVariant::CallHead { entity, tokens },
        );

        fn routine_call_head_tokens<'eval>(
            text: &Text,
            routine_keyword: &'static str,
            ident: Identifier,
            parameters: &[Parameter],
        ) -> Vec<TraceTokenData> {
            let mut tokens = vec![
                keyword!(routine_keyword),
                ident!(ident.as_str()),
                special!("("),
            ];
            for i in 0..parameters.len() {
                let input_placeholder = &parameters[i];
                tokens.push(ident!(input_placeholder.ranged_ident.ident.as_str()));
                tokens.push(special!(": "));
                tokens.push(route!(text.ranged(input_placeholder.ranged_ty.range)));
                if i < parameters.len() - 1 {
                    tokens.push(special!(", "));
                }
            }
            tokens.push(special!("):"));
            tokens
        }
    }
}
