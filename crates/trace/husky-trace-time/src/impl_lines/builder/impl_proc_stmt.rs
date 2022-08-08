use vm::FrameKind;

use super::*;

impl<'a> TraceTokenBuilder<'a> {
    pub(crate) fn proc_stmt_tokens(&mut self, stmt: &ProcStmt, history: &Arc<History<'static>>) {
        match stmt.variant {
            ProcStmtVariant::Init {
                varname,
                ref initial_value,
                init_kind,
            } => {
                self.push(keyword!(match init_kind {
                    InitKind::Let => "let ",
                    InitKind::Var => "var ",
                    InitKind::Decl => panic!(),
                }));
                self.push(ident!(varname.ident.0));
                self.push(special!(" = "));
                self.eager_expr_tokens(initial_value, history, ExprTokenConfig::stmt())
            }
            ProcStmtVariant::Assert { ref condition } => {
                self.push(keyword!("assert "));
                self.eager_expr_tokens(condition, history, ExprTokenConfig::stmt())
            }
            ProcStmtVariant::Execute { ref expr } => {
                self.eager_expr_tokens(expr, history, ExprTokenConfig::exec());
                match expr.variant {
                    EagerExprVariant::Opn {
                        ref opn_variant, ..
                    } => match opn_variant {
                        EagerOpnVariant::Binary { opr, this_ty: this } => match opr {
                            BinaryOpr::Assign(_) => {
                                self.push(fade!(" = "));
                                if let Some(register_result) = history.register_result(expr) {
                                    todo!()
                                    // self.push(register_result.into())
                                } else {
                                    todo!()
                                }
                            }
                            BinaryOpr::Pure(_) => (),
                        },
                        _ => (),
                    },
                    _ => panic!(),
                }
            }
            ProcStmtVariant::Return { ref result, .. } => {
                self.push(keyword!("return "));
                self.eager_expr_tokens(result, history, ExprTokenConfig::stmt())
            }
            ProcStmtVariant::ConditionFlow { ref branches } => todo!(),
            ProcStmtVariant::Loop {
                ref loop_variant, ..
            } => self.loop_stmt_tokens(stmt, loop_variant, history),
            ProcStmtVariant::Break => self.push(keyword!("break")),
            ProcStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => todo!(),
        }
    }

    fn loop_stmt_tokens(
        &mut self,
        stmt: &ProcStmt,
        loop_variant: &LoopVariant,
        history: &Arc<History<'static>>,
    ) {
        match loop_variant {
            LoopVariant::For {
                frame_var,
                ref initial_boundary,
                ref final_boundary,
                ..
            } => {
                self.push(keyword!("for "));
                self.initial_boundary_tokens(initial_boundary, history);
                self.push(ident!(frame_var.ident.0));
                self.final_boundary_tokens(final_boundary, history);
                self.push(special!(":"));
            }
            LoopVariant::ForExt {
                frame_var,
                ref final_boundary,
                ..
            } => {
                self.push(keyword!("forext "));
                self.push(ident!(frame_var.ident.0));
                self.final_boundary_tokens(final_boundary, history);
                self.push(special!(":"));
            }
            LoopVariant::While { ref condition } => {
                self.push(keyword!("while "));
                self.eager_expr_tokens(condition, history, ExprTokenConfig::loop_head());
                self.push(special!(":"));
            }
            LoopVariant::DoWhile { condition } => {
                self.push(keyword!("do while "));
                self.eager_expr_tokens(condition, history, ExprTokenConfig::loop_head());
                self.push(special!(":"));
            }
        }
        if let Some(entry) = history.get(stmt) {
            match entry {
                HistoryEntry::Loop {
                    control, mutations, ..
                } => self.add_control_tokens(control),
                _ => panic!(),
            }
        }
    }

    fn initial_boundary_tokens(&mut self, boundary: &Boundary, history: &Arc<History<'static>>) {
        match boundary.opt_bound {
            Some(ref bound) => {
                self.eager_expr_tokens(bound, history, ExprTokenConfig::stmt());
                match boundary.kind {
                    BoundaryKind::UpperOpen => self.push(special!(" > ")),
                    BoundaryKind::UpperClosed => self.push(special!(" >= ")),
                    BoundaryKind::LowerOpen => self.push(special!(" < ")),
                    BoundaryKind::LowerClosed => self.push(special!(" <= ")),
                }
            }
            None => (),
        }
    }

    fn final_boundary_tokens(&mut self, boundary: &Boundary, history: &Arc<History<'static>>) {
        match boundary.opt_bound {
            Some(ref bound) => {
                vec![special!(match boundary.kind {
                    BoundaryKind::UpperOpen => " < ",
                    BoundaryKind::UpperClosed => " <= ",
                    BoundaryKind::LowerOpen => " > ",
                    BoundaryKind::LowerClosed => " >= ",
                })];
                self.eager_expr_tokens(bound, history, ExprTokenConfig::stmt())
            }
            None => (),
        }
    }

    pub(crate) fn loop_frame_tokens(&mut self, loop_frame_data: &LoopFrameData) {
        match loop_frame_data.frame_kind {
            FrameKind::For(frame_var) => self.extend([
                keyword!("frame "),
                ident!(frame_var.0),
                special!(" = "),
                literal!(format!("{}", loop_frame_data.frame_var_value)),
            ]),
            FrameKind::Loop => self.extend([
                keyword!("frame "),
                literal!(format!("{}", loop_frame_data.frame_var_value)),
            ]),
        };
        self.add_control_tokens(&loop_frame_data.control)
    }

    pub(crate) fn proc_branch_tokens(
        &mut self,
        stmt: &ProcStmt,
        branch: &ProcConditionFlowBranch,
        history: &Arc<History<'static>>,
    ) {
        match branch.variant {
            ProcConditionFlowBranchVariant::If { ref condition } => {
                self.push(keyword!("if "));
                self.eager_expr_tokens(condition, history, ExprTokenConfig::branch());
                self.push(special!(":"))
            }
            ProcConditionFlowBranchVariant::Elif { ref condition } => {
                self.push(keyword!("elif "));
                self.eager_expr_tokens(condition, history, ExprTokenConfig::branch());
                self.push(special!(":"))
            }
            ProcConditionFlowBranchVariant::Else => {
                self.push(keyword!("else"));
                self.push(special!(":"))
            }
        }
        if let Some(entry) = history.get(stmt) {
            match entry {
                HistoryEntry::ControlFlow {
                    opt_branch_entered,
                    vm_branches,
                    control,
                    stack_snapshot,
                    mutations,
                } => self.add_control_tokens(control),
                _ => todo!(),
            }
        }
    }
}
