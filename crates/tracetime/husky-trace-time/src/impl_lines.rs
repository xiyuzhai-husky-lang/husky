mod expr_config;
mod impl_eager_expr;
mod impl_feature_branch;
mod impl_feature_expr;
mod impl_feature_stmt;
mod impl_func_stmt;
mod impl_proc_stmt;
mod impl_token;

pub use expr_config::*;

use super::*;

impl HuskyTracetime {
    pub(crate) fn trace_lines(
        &mut self,
        indent: Indent,
        trace_variant: &TraceVariant<'static>,
        has_parent: bool,
    ) -> Vec<TraceLineData> {
        TraceLineBuilder::new(self, indent, trace_variant, has_parent).build()
    }
}
pub struct TraceLineBuilder<'a> {
    trace_time: &'a mut HuskyTracetime,
    trace_variant: &'a TraceVariant<'static>,
    indent: Indent,
    has_parent: bool,
    lines: Vec<TraceLineData>,
}

impl<'a> TraceLineBuilder<'a> {
    pub(super) fn new(
        trace_time: &'a mut HuskyTracetime,
        indent: Indent,
        trace_variant: &'a TraceVariant<'static>,
        has_parent: bool,
    ) -> Self {
        TraceLineBuilder {
            trace_time,
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

impl<'a> std::ops::Deref for TraceLineBuilder<'a> {
    type Target = HuskyTracetime;

    fn deref(&self) -> &Self::Target {
        self.trace_time
    }
}

impl<'a> std::ops::DerefMut for TraceLineBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.trace_time
    }
}

impl<'a> TraceLineBuilder<'a> {
    pub(super) fn build(mut self) -> Vec<TraceLineData> {
        match self.trace_variant {
            TraceVariant::Main(_) => self.push_keyword("main"),
            TraceVariant::EntityFeature {
                route, ref repr, ..
            } => {
                if let Some(token) = repr.opt_leading_keyword() {
                    self.gen_keyword_token(token, None);
                    self.gen_ident_token(route.ident().as_str(), None)
                } else {
                    self.gen_keyword_token(route.ident().as_str(), None)
                }
            }
            TraceVariant::Module { route, .. } => {
                self.push(TraceTokenData {
                    kind: TraceTokenKind::Mod,
                    value: "mod ".into(),
                    opt_associated_trace_id: None,
                });
                self.gen_ident_token(route.ident().as_str(), None)
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
                history,
                ..
            } => self.func_branch_tokens(stmt, branch, history),
            TraceVariant::ProcBranch {
                stmt,
                branch,
                history,
                ..
            } => self.proc_branch_tokens(stmt, branch, history),
            TraceVariant::FeatureCallArgument {
                name: ident,
                argument: input,
            } => {
                self.gen_feature_expr_tokens(input, ExprTokenConfig::expr(true));
                let first_line = self.lines.first_mut().unwrap();
                first_line.tokens.insert(0, trace_token_special!(" = "));
                first_line.tokens.insert(0, ident!(*ident))
            }
            TraceVariant::EagerCallArgument {
                name,
                ref argument,
                ref history,
            } => {
                self.gen_ident_token(name, None);
                self.gen_assign_token();
                self.eager_expr_tokens(argument, history, ExprTokenConfig::expr(true));
            }
        }
        self.lines
    }

    fn extend(&mut self, tokens: impl IntoIterator<Item = TraceTokenData>) {
        self.lines.last_mut().unwrap().tokens.extend(tokens)
    }

    fn add_control_tokens(&mut self, control: &ControlSnapshot) {
        match control {
            ControlSnapshot::None => (),
            ControlSnapshot::Return(_) => {
                self.push(fade!(" = "));
                self.gen_keyword_token("return", None);
                todo!()
                // self.push(value.snapshot().into());
            }
            ControlSnapshot::Break => {
                self.push(fade!(" = "));
                self.gen_keyword_token("break", None);
            }
            ControlSnapshot::Err(ref e) => {
                self.push(fade!(" = "));
                self.push(TraceTokenData {
                    kind: TraceTokenKind::Error,
                    value: e.message().to_string(),
                    opt_associated_trace_id: None,
                });
            }
        }
    }
}
