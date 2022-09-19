mod expr_config;
mod impl_eager_expr;
mod impl_feature_branch;
mod impl_feature_expr;
mod impl_feature_stmt;
mod impl_func_stmt;
mod impl_proc_stmt;
mod impl_token;

pub use expr_config::*;
use husky_text::Row;

use super::*;

impl Tracetime {
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
    debugtime: &'a mut Tracetime,
    trace_variant: &'a TraceVariant<'static>,
    has_parent: bool,
    lines: Vec<TraceLineData>,
    opt_cur_row: Option<Row>,
}

impl<'a> TraceLineBuilder<'a> {
    pub(super) fn new(
        debugtime: &'a mut Tracetime,
        indent: Indent,
        trace_variant: &'a TraceVariant<'static>,
        has_parent: bool,
    ) -> Self {
        TraceLineBuilder {
            debugtime,
            trace_variant,
            has_parent,
            lines: vec![TraceLineData {
                indent,
                tokens: vec![],
                idx: 0,
            }],
            opt_cur_row: None,
        }
    }
}

impl<'a> std::ops::Deref for TraceLineBuilder<'a> {
    type Target = Tracetime;

    fn deref(&self) -> &Self::Target {
        self.debugtime
    }
}

impl<'a> std::ops::DerefMut for TraceLineBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.debugtime
    }
}

impl<'a> TraceLineBuilder<'a> {
    pub(super) fn build(mut self) -> Vec<TraceLineData> {
        match self.trace_variant {
            TraceVariant::Main(_) => self.gen_keyword_token("main", None, None),
            TraceVariant::EntityFeature {
                route, ref repr, ..
            } => {
                if let Some(token) = repr.opt_leading_keyword() {
                    self.gen_keyword_token(token, None, None);
                    self.gen_ident_token(route.ident().as_str(), None, None)
                } else {
                    self.gen_keyword_token(route.ident().as_str(), None, None)
                }
            }
            TraceVariant::Module { route, .. } => {
                self.gen_mod();
                self.gen_ident_token(route.ident().as_str(), None, None)
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
            } => self.gen_eager_expr_tokens(expr, history, ExprTokenConfig::expr(self.has_parent)),
            TraceVariant::CallHead { ref tokens, .. } => self.extend(tokens.clone()),
            TraceVariant::LoopFrame {
                ref loop_frame_data,
                ..
            } => self.gen_loop_frame_tokens(loop_frame_data),
            TraceVariant::FuncBranch {
                stmt,
                branch,
                history,
                ..
            } => self.gen_func_branch_tokens(stmt, branch, history),
            TraceVariant::ProcBranch {
                stmt,
                branch,
                history,
                ..
            } => self.gen_proc_branch_tokens(stmt, branch, history),
            TraceVariant::FeatureCallArgument { name, argument } => {
                self.gen_ident_token(name, None, None);
                self.gen_assign_token();
                self.gen_feature_expr_tokens(argument, ExprTokenConfig::expr(true))
            }
            TraceVariant::EagerCallArgument {
                name,
                ref argument,
                ref history,
            } => {
                self.gen_ident_token(name, None, None);
                self.gen_assign_token();
                self.gen_eager_expr_tokens(argument, history, ExprTokenConfig::expr(true));
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
                self.gen_fade_assign_token();
                self.gen_keyword_token("return", None, None);
                todo!()
                // self.push(value.snapshot().into());
            }
            ControlSnapshot::Break => {
                self.gen_fade_assign_token();
                self.gen_keyword_token("break", None, None);
            }
            ControlSnapshot::Err(ref e) => {
                self.gen_fade_assign_token();
                self.gen_error_token(e, None)
            }
        }
    }
}
