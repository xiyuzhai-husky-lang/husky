use super::*;

impl HuskyTraceTime {
    pub(crate) fn proc_stmt_tokens(
        &mut self,
        stmt: &ProcStmt,
        history: &Arc<History<'static>>,
    ) -> Vec<TraceTokenData> {
        match stmt.variant {
            ProcStmtVariant::Init {
                varname,
                ref initial_value,
                init_kind,
            } => {
                let mut tokens = vec![keyword!(match init_kind {
                    vm::InitKind::Let => "let ",
                    vm::InitKind::Var => "var ",
                    vm::InitKind::Decl => panic!(),
                })];
                tokens.push(ident!(varname.ident.0));
                tokens.push(special!(" = "));
                tokens.extend(self.eager_expr_tokens(
                    initial_value,
                    history,
                    ExprTokenConfig::stmt(),
                ));
                tokens
            }
            ProcStmtVariant::Assert { ref condition } => {
                let mut tokens = vec![keyword!("assert ")];
                tokens.extend(self.eager_expr_tokens(condition, history, ExprTokenConfig::stmt()));
                tokens
            }
            ProcStmtVariant::Execute { ref expr } => {
                let mut tokens = self.eager_expr_tokens(expr, history, ExprTokenConfig::exec());
                match expr.variant {
                    EagerExprVariant::Opn {
                        ref opn_variant, ..
                    } => match opn_variant {
                        EagerOpnVariant::Binary { opr, this_ty: this } => match opr {
                            BinaryOpr::Assign(_) => {
                                tokens.push(fade!(" = "));
                                tokens.push(history.value_result(expr).into())
                            }
                            BinaryOpr::Pure(_) => (),
                        },
                        _ => (),
                    },
                    _ => panic!(),
                }
                tokens
            }
            ProcStmtVariant::Return { ref result } => {
                let mut tokens = vec![keyword!("return ")];
                tokens.extend(self.eager_expr_tokens(result, history, ExprTokenConfig::stmt()));
                tokens
            }
            ProcStmtVariant::ConditionFlow { ref branches } => todo!(),
            ProcStmtVariant::Loop {
                loop_variant: ref loop_kind,
                ref stmts,
            } => match loop_kind {
                LoopVariant::For {
                    frame_var,
                    ref initial_boundary,
                    ref final_boundary,
                    ..
                } => {
                    let mut tokens = vec![keyword!("for ")];
                    tokens.extend(self.initial_boundary_tokens(initial_boundary, history));
                    tokens.push(ident!(frame_var.ident.0));
                    tokens.extend(self.final_boundary_tokens(final_boundary, history));
                    tokens.push(special!(":"));
                    tokens
                }
                LoopVariant::ForExt {
                    frame_var,
                    ref final_boundary,
                    ..
                } => {
                    let mut tokens = vec![keyword!("forext ")];
                    tokens.push(ident!(frame_var.ident.0));
                    tokens.extend(self.final_boundary_tokens(final_boundary, history));
                    tokens.push(special!(":"));
                    tokens
                }
                LoopVariant::While { ref condition } => {
                    let mut tokens = vec![keyword!("while ")];
                    tokens.extend(self.eager_expr_tokens(
                        condition,
                        history,
                        ExprTokenConfig::loop_head(),
                    ));
                    tokens.push(special!(":"));
                    tokens
                }
                LoopVariant::DoWhile { condition } => {
                    let mut tokens = vec![keyword!("do while ")];
                    tokens.extend(self.eager_expr_tokens(
                        condition,
                        history,
                        ExprTokenConfig::loop_head(),
                    ));
                    tokens.push(special!(":"));
                    tokens
                }
            },
            ProcStmtVariant::Break => vec![keyword!("break")],
            ProcStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => todo!(),
        }
    }

    fn initial_boundary_tokens(
        &mut self,
        boundary: &Boundary,
        history: &Arc<History<'static>>,
    ) -> Vec<TraceTokenData> {
        match boundary.opt_bound {
            Some(ref bound) => {
                let mut tokens = self.eager_expr_tokens(bound, history, ExprTokenConfig::stmt());
                match boundary.kind {
                    BoundaryKind::UpperOpen => tokens.push(special!(" > ")),
                    BoundaryKind::UpperClosed => tokens.push(special!(" >= ")),
                    BoundaryKind::LowerOpen => tokens.push(special!(" < ")),
                    BoundaryKind::LowerClosed => tokens.push(special!(" <= ")),
                }
                tokens
            }
            None => vec![],
        }
    }

    fn final_boundary_tokens(
        &mut self,
        boundary: &Boundary,
        history: &Arc<History<'static>>,
    ) -> Vec<TraceTokenData> {
        match boundary.opt_bound {
            Some(ref bound) => {
                let mut tokens = vec![special!(match boundary.kind {
                    BoundaryKind::UpperOpen => " < ",
                    BoundaryKind::UpperClosed => " <= ",
                    BoundaryKind::LowerOpen => " > ",
                    BoundaryKind::LowerClosed => " >= ",
                })];
                tokens.extend(self.eager_expr_tokens(bound, history, ExprTokenConfig::stmt()));
                tokens
            }
            None => vec![],
        }
    }

    pub(crate) fn loop_frame_tokens(&self, vm_loop_frame: &LoopFrameData) -> Vec<TraceTokenData> {
        match vm_loop_frame.frame_kind {
            vm::FrameKind::For(frame_var) => {
                vec![
                    keyword!("frame "),
                    ident!(frame_var.0),
                    special!(" = "),
                    literal!(format!("{}", vm_loop_frame.frame_var_value)),
                ]
            }
            vm::FrameKind::Loop => {
                vec![
                    keyword!("frame "),
                    literal!(format!("{}", vm_loop_frame.frame_var_value)),
                ]
            }
        }
    }

    pub(crate) fn proc_branch_tokens(
        &mut self,
        indent: Indent,
        branch: &ProcConditionBranch,
        history: &Arc<History<'static>>,
    ) -> Vec<TraceTokenData> {
        let mut tokens = Vec::new();
        match branch.variant {
            ProcConditionBranchVariant::If { ref condition } => {
                tokens.push(keyword!("if "));
                tokens.extend(self.eager_expr_tokens(
                    condition,
                    history,
                    ExprTokenConfig::branch(),
                ));
                tokens.push(special!(":"))
            }
            ProcConditionBranchVariant::Elif { ref condition } => {
                tokens.push(keyword!("elif "));
                tokens.extend(self.eager_expr_tokens(
                    condition,
                    history,
                    ExprTokenConfig::branch(),
                ));
                tokens.push(special!(":"))
            }
            ProcConditionBranchVariant::Else => {
                tokens.push(keyword!("else"));
                tokens.push(special!(":"))
            }
        }
        tokens
    }
}
