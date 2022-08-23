mod impl_eager_expr;
mod impl_feature_branch;
mod impl_feature_expr;
mod impl_feature_stmt;
mod impl_func_stmt;
mod impl_proc_stmt;

use super::*;

pub struct TraceTokenBuilder<'a> {
    trace_time: &'a mut HuskyTraceTime,
    trace_id: TraceId,
    trace_variant: &'a TraceVariant<'static>,
    indent: Indent,
    has_parent: bool,
    lines: Vec<TraceLineData>,
}

impl<'a> TraceTokenBuilder<'a> {
    pub(super) fn new(
        trace_time: &'a mut HuskyTraceTime,
        trace_id: TraceId,
        indent: Indent,
        trace_variant: &'a TraceVariant<'static>,
        has_parent: bool,
    ) -> Self {
        TraceTokenBuilder {
            trace_time,
            trace_id,
            trace_variant,
            indent,
            has_parent,
            lines: vec![TraceLineData {
                indent,
                tokens: vec![],
                idx: 0,
            }],
        }
    }
}

impl<'a> std::ops::Deref for TraceTokenBuilder<'a> {
    type Target = HuskyTraceTime;

    fn deref(&self) -> &Self::Target {
        self.trace_time
    }
}

impl<'a> std::ops::DerefMut for TraceTokenBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.trace_time
    }
}

impl<'a> TraceTokenBuilder<'a> {
    pub(super) fn build(mut self) -> Vec<TraceLineData> {
        match self.trace_variant {
            TraceVariant::Main(feature_block) => self.push(keyword!("main")),
            TraceVariant::EntityFeature {
                route, ref repr, ..
            } => {
                if let Some(token) = repr.opt_leading_keyword() {
                    self.push(keyword!(token));
                    self.push(ident!(route.ident().as_str()))
                } else {
                    self.push(keyword!(route.ident().as_str()))
                }
            }
            TraceVariant::Module { route, .. } => {
                self.push(TraceTokenData {
                    kind: TraceTokenKind::Mod,
                    value: "mod ".into(),
                    opt_associated_trace_id: None,
                });
                self.push(ident!(route.ident().as_str()))
            }
            TraceVariant::FeatureStmt(stmt) => self.feature_stmt_tokens(stmt),
            TraceVariant::FeatureExpr(ref expr) => {
                self.gen_feature_expr_tokens(expr, ExprTokenConfig::expr(false))
            }
            TraceVariant::FeatureBranch(branch) => self.feature_branch_tokens(branch),
            TraceVariant::FuncStmt {
                ref stmt,
                ref history,
                ..
            } => self.func_stmt_tokens(stmt, history),
            TraceVariant::ProcStmt {
                ref stmt,
                ref history,
            } => self.proc_stmt_tokens(stmt, history),
            TraceVariant::EagerExpr {
                ref expr,
                ref history,
            } => self.eager_expr_tokens(expr, history, ExprTokenConfig::expr(self.has_parent)),
            TraceVariant::CallHead { ref tokens, .. } => self.extend(tokens.clone()),
            TraceVariant::LoopFrame {
                ref loop_frame_data,
                ..
            } => self.loop_frame_tokens(loop_frame_data),
            TraceVariant::FuncBranch {
                stmt,
                branch,
                branch_idx,
                history,
                ..
            } => self.func_branch_tokens(stmt, branch, history),
            TraceVariant::ProcBranch {
                stmt,
                branch,
                branch_idx,
                history,
                ..
            } => self.proc_branch_tokens(stmt, branch, history),
            TraceVariant::FeatureCallArgument {
                name: ident,
                argument: input,
            } => {
                self.gen_feature_expr_tokens(input, ExprTokenConfig::expr(true));
                let first_line = self.lines.first_mut().unwrap();
                first_line.tokens.insert(0, special!(" = "));
                first_line.tokens.insert(0, ident!(*ident))
            }
            TraceVariant::EagerCallArgument {
                name,
                ref argument,
                ref history,
            } => {
                self.eager_expr_tokens(argument, history, ExprTokenConfig::expr(true));
                let first_line = self.lines.first_mut().unwrap();
                first_line.tokens.insert(0, special!(" = "));
                first_line.tokens.insert(0, ident!(*name));
            }
        }
        self.lines
    }

    fn push(&mut self, token: TraceTokenData) {
        self.lines.last_mut().unwrap().tokens.push(token)
    }
    fn extend(&mut self, tokens: impl IntoIterator<Item = TraceTokenData>) {
        self.lines.last_mut().unwrap().tokens.extend(tokens)
    }

    fn add_control_tokens(&mut self, control: &ControlSnapshot) {
        match control {
            ControlSnapshot::None => (),
            ControlSnapshot::Return(ref value) => {
                self.push(fade!(" = "));
                self.push(keyword!("return"));
                todo!()
                // self.push(value.snapshot().into());
            }
            ControlSnapshot::Break => {
                self.push(fade!(" = "));
                self.push(keyword!("break"));
            }
            ControlSnapshot::Err(ref e) => {
                self.push(fade!(" = "));
                self.push(
                    todo!(), // e.clone().into()
                );
            }
        }
    }
}
