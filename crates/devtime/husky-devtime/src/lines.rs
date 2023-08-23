// mod call_head;
// mod eager_expr;
// mod expr_config;
// mod feature_branch;
// mod feature_expr;
// mod feature_stmt;
// mod func_stmt;
// mod proc_stmt;
// mod token;

// pub use self::expr_config::*;
use husky_text::TextLine;

use super::*;

impl Devtime {
    pub(crate) fn trace_lines(
        &mut self,
        indent: Indent,
        trace_variant: &TraceVariant,
        has_parent: bool,
    ) -> Vec<TraceLineData> {
        TraceLineGenerator::new(self, indent, trace_variant, has_parent).gen()
    }
}
pub struct TraceLineGenerator<'a> {
    devtime: &'a mut Devtime,
    trace_variant: &'a TraceVariant,
    has_parent: bool,
    lines: Vec<TraceLineData>,
    current_line: Option<TextLine>,
}

impl<'a> TraceLineGenerator<'a> {
    pub(super) fn new(
        devtime: &'a mut Devtime,
        indent: Indent,
        trace_variant: &'a TraceVariant,
        has_parent: bool,
    ) -> Self {
        TraceLineGenerator {
            devtime,
            trace_variant,
            has_parent,
            lines: vec![TraceLineData {
                indent,
                tokens: vec![],
                idx: 0,
            }],
            current_line: None,
        }
    }
}

impl<'a> std::ops::Deref for TraceLineGenerator<'a> {
    type Target = Devtime;

    fn deref(&self) -> &Self::Target {
        self.devtime
    }
}

impl<'a> std::ops::DerefMut for TraceLineGenerator<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.devtime
    }
}

impl<'a> TraceLineGenerator<'a> {
    pub(super) fn gen(mut self) -> Vec<TraceLineData> {
        todo!()
        // match self.trace_variant {
        //     TraceVariant::Main(..) => self.render_keyword_token("main", None, None),
        //     TraceVariant::EntityFeature {
        //         route, ref repr, ..
        //     } => {
        //         if let Some(token) = repr.opt_leading_keyword() {
        //             self.render_keyword_token(token, None, None);
        //             self.render_ident_token(route.ident().as_str(), None, None)
        //         } else {
        //             self.render_keyword_token(route.ident().as_str(), None, None)
        //         }
        //     }
        //     TraceVariant::Module { route, .. } => {
        //         self.gen_mod();
        //         self.render_ident_token(route.ident().as_str(), None, None)
        //     }
        //     TraceVariant::FeatureStmt(stmt) => self.feature_stmt_tokens(stmt),
        //     TraceVariant::FeatureExpr(ref expr) => {
        //         self.gen_feature_expr(expr, ExprTokenConfig::expr(false))
        //     }
        //     TraceVariant::FeatureBranch(branch) => self.gen_feature_branch(branch),
        //     TraceVariant::FuncStmt {
        //         ref stmt,
        //         ref history,
        //         ..
        //     } => self.func_stmt_tokens(stmt, history),
        //     TraceVariant::ProcStmt {
        //         ref stmt,
        //         ref history,
        //     } => self.proc_stmt_tokens(stmt, history),
        //     TraceVariant::EagerExpr {
        //         ref expr,
        //         ref history,
        //     } => self.gen_eager_expr_tokens(expr, history, ExprTokenConfig::expr(self.has_parent)),
        //     TraceVariant::CallHead { ref item, .. } => self.gen_call_head_lines(item),
        //     TraceVariant::LoopFrame {
        //         ref loop_frame_data,
        //         ..
        //     } => self.gen_loop_frame_tokens(loop_frame_data),
        //     TraceVariant::FuncBranch {
        //         stmt,
        //         branch,
        //         history,
        //         ..
        //     } => self.gen_func_branch_tokens(stmt, branch, history),
        //     TraceVariant::ProcBranch {
        //         stmt,
        //         branch,
        //         history,
        //         ..
        //     } => self.gen_proc_branch_tokens(stmt, branch, history),
        //     TraceVariant::FeatureCallArgument { name, argument } => {
        //         self.render_ident_token(name, None, None);
        //         self.gen_assign_token();
        //         self.gen_feature_expr(argument, ExprTokenConfig::expr(true))
        //     }
        //     TraceVariant::EagerCallArgument {
        //         name,
        //         ref argument,
        //         ref history,
        //     } => {
        //         self.render_ident_token(name, None, None);
        //         self.gen_assign_token();
        //         self.gen_eager_expr_tokens(argument, history, ExprTokenConfig::expr(true));
        //     }
        // }
        // self.lines
    }

    fn extend(&mut self, tokens: impl IntoIterator<Item = TraceTokenData>) {
        self.lines.last_mut().unwrap().tokens.extend(tokens)
    }

    fn add_control_tokens(&mut self, control: &ControlSnapshot) {
        match control {
            ControlSnapshot::None => (),
            ControlSnapshot::Return(_) => {
                self.gen_fade_assign_token();
                self.render_keyword_token("return", None, None);
                todo!()
                // self.push(value.snapshot().into());
            }
            ControlSnapshot::Break => {
                self.gen_fade_assign_token();
                self.render_keyword_token("break", None, None);
            }
            ControlSnapshot::Err(ref e) => {
                self.gen_fade_assign_token();
                self.gen_error_token(e, None)
            }
        }
    }
}
