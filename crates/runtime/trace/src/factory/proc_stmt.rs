use compile_time_db::HuskyLangCompileTime;
use print_utils::epin;
use text::Text;
use upcast::Upcast;
use vm::{
    exec_debug, exec_loop_debug, BinaryOpr, BoundaryKind, History, HistoryEntry, InstructionSheet,
    LoopFrameData, StackSnapshot, VMLoopKind,
};

use super::{expr::ExprTokenConfig, *};
use crate::*;

impl<'eval> TraceFactory<'eval> {
    fn new_proc_stmt_trace(
        &self,
        parent_id: TraceId,
        indent: Indent,
        stmt: Arc<ProcStmt>,
        text: &Text,
        history: Arc<History<'eval>>,
    ) -> Arc<Trace<'eval>> {
        self.new_trace(
            Some(parent_id),
            indent,
            TraceVariant::ProcStmt { stmt, history },
            text,
        )
    }

    fn new_proc_branch_trace(
        &self,
        text: &Text,
        parent_id: TraceId,
        indent: Indent,
        stmt: Arc<ProcStmt>,
        branch: Arc<ProcConditionBranch>,
        branch_idx: u8,
        history: Arc<History<'eval>>,
    ) -> Arc<Trace<'eval>> {
        let opt_vm_branch = history.get(&stmt).map(|entry| match entry {
            HistoryEntry::ConditionFlow { vm_branches, .. } => {
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
            text,
        )
    }

    pub(super) fn proc_stmt_lines(
        &self,
        stmt: &ProcStmt,
        text: &Text,
        history: &Arc<History<'eval>>,
    ) -> Vec<LineProps<'eval>> {
        vec![LineProps {
            indent: stmt.indent,
            tokens: self.proc_stmt_tokens(stmt, text, history),
            idx: 0,
        }]
    }

    pub(super) fn proc_stmt_tokens(
        &self,
        stmt: &ProcStmt,
        text: &Text,
        history: &Arc<History<'eval>>,
    ) -> Vec<TokenProps<'eval>> {
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
                    text,
                    history,
                    ExprTokenConfig::stmt(),
                ));
                tokens
            }
            ProcStmtVariant::Assert { ref condition } => {
                let mut tokens = vec![keyword!("assert ")];
                tokens.extend(self.eager_expr_tokens(
                    condition,
                    text,
                    history,
                    ExprTokenConfig::stmt(),
                ));
                tokens
            }
            ProcStmtVariant::Execute { ref expr } => {
                let mut tokens =
                    self.eager_expr_tokens(expr, text, history, ExprTokenConfig::exec());
                match expr.variant {
                    EagerExprVariant::Opn {
                        ref opn_variant, ..
                    } => match opn_variant {
                        EagerOpnVariant::Binary { opr, this_ty: this } => match opr {
                            BinaryOpr::Assign(_) => {
                                tokens.push(fade!(" = "));
                                tokens.push(history.value(expr).into())
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
                tokens.extend(self.eager_expr_tokens(
                    result,
                    text,
                    history,
                    ExprTokenConfig::stmt(),
                ));
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
                    tokens.extend(self.initial_boundary_tokens(initial_boundary, text, history));
                    tokens.push(ident!(frame_var.ident.0));
                    tokens.extend(self.final_boundary_tokens(final_boundary, text, history));
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
                    tokens.extend(self.final_boundary_tokens(final_boundary, text, history));
                    tokens.push(special!(":"));
                    tokens
                }
                LoopVariant::While { ref condition } => {
                    let mut tokens = vec![keyword!("while ")];
                    tokens.extend(self.eager_expr_tokens(
                        condition,
                        text,
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
                        text,
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
        &self,
        boundary: &Boundary,
        text: &Text,
        history: &Arc<History<'eval>>,
    ) -> Vec<TokenProps<'eval>> {
        match boundary.opt_bound {
            Some(ref bound) => {
                let mut tokens =
                    self.eager_expr_tokens(bound, text, history, ExprTokenConfig::stmt());
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
        &self,
        boundary: &Boundary,
        text: &Text,
        history: &Arc<History<'eval>>,
    ) -> Vec<TokenProps<'eval>> {
        match boundary.opt_bound {
            Some(ref bound) => {
                let mut tokens = vec![special!(match boundary.kind {
                    BoundaryKind::UpperOpen => " < ",
                    BoundaryKind::UpperClosed => " <= ",
                    BoundaryKind::LowerOpen => " > ",
                    BoundaryKind::LowerClosed => " >= ",
                })];
                tokens.extend(self.eager_expr_tokens(
                    bound,
                    text,
                    history,
                    ExprTokenConfig::stmt(),
                ));
                tokens
            }
            None => vec![],
        }
    }

    pub fn proc_stmts_traces<'a>(
        &'a self,
        parent_id: TraceId,
        indent: Indent,
        stmts: &'a [Arc<ProcStmt>],
        text: &'a Text,
        history: &'a Arc<History<'eval>>,
    ) -> Vec<Arc<Trace<'eval>>> {
        let mut traces = Vec::new();
        for stmt in stmts {
            match stmt.variant {
                ProcStmtVariant::ConditionFlow { ref branches } => {
                    for (branch_idx, branch) in branches.iter().enumerate() {
                        traces.push(self.new_proc_branch_trace(
                            text,
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
                    text,
                    history.clone(),
                )),
            }
        }
        traces
    }

    pub(super) fn loop_subtraces(
        &self,
        compile_time: &HuskyLangCompileTime,
        parent: &Trace,
        loop_kind: VMLoopKind,
        loop_stmt: &Arc<ProcStmt>,
        body_stmts: &Arc<Vec<Arc<ProcStmt>>>,
        text: &Text,
        stack_snapshot: &StackSnapshot<'eval>,
        body_instruction_sheet: &Arc<InstructionSheet>,
    ) -> Arc<Vec<Arc<Trace<'eval>>>> {
        let frames = exec_loop_debug(
            compile_time.upcast(),
            loop_kind,
            &body_instruction_sheet,
            stack_snapshot,
        );
        Arc::new(
            frames
                .into_iter()
                .map(|loop_frame_data| {
                    self.new_trace(
                        Some(parent.id),
                        parent.indent + 2,
                        TraceVariant::LoopFrame {
                            loop_stmt: loop_stmt.clone(),
                            body_stmts: body_stmts.clone(),
                            body_instruction_sheet: body_instruction_sheet.clone(),
                            loop_frame_data,
                        },
                        text,
                    )
                })
                .collect(),
        )
    }

    pub(super) fn loop_frame_subtraces(
        &self,
        compile_time: &HuskyLangCompileTime,
        text: &Text,
        loop_stmt: &Arc<ProcStmt>,
        stmts: &[Arc<ProcStmt>],
        instruction_sheet: &InstructionSheet,
        loop_frame_data: &LoopFrameData<'eval>,
        parent: &Trace,
    ) -> Avec<Trace<'eval>> {
        let history = exec_debug(
            compile_time.upcast(),
            instruction_sheet,
            &loop_frame_data.stack_snapshot,
        );
        let mut subtraces: Vec<_> =
            self.proc_stmts_traces(parent.id, parent.indent + 2, stmts, text, &history);
        match loop_stmt.variant {
            ProcStmtVariant::Loop {
                ref loop_variant, ..
            } => match loop_variant {
                LoopVariant::For { .. } | LoopVariant::ForExt { .. } => (),
                LoopVariant::While { condition } => subtraces.insert(
                    0,
                    self.new_eager_expr_trace(
                        text,
                        condition.clone(),
                        history.clone(),
                        Some(parent),
                        parent.indent + 2,
                    ),
                ),
                LoopVariant::DoWhile { condition } => subtraces.push(self.new_eager_expr_trace(
                    text,
                    condition.clone(),
                    history.clone(),
                    Some(parent),
                    parent.indent + 2,
                )),
            },
            _ => panic!(),
        }
        Arc::new(subtraces)
    }

    pub(super) fn loop_frame_lines(
        &self,
        indent: Indent,
        loop_frame_data: &LoopFrameData,
    ) -> Vec<LineProps<'eval>> {
        vec![LineProps {
            indent,
            tokens: self.loop_frame_tokens(loop_frame_data),
            idx: 0,
        }]
    }

    pub(super) fn loop_frame_tokens(
        &self,
        vm_loop_frame: &LoopFrameData,
    ) -> Vec<TokenProps<'eval>> {
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

    pub(super) fn proc_branch_subtraces(
        &self,
        compile_time: &HuskyLangCompileTime,
        text: &Text,
        stmts: &[Arc<ProcStmt>],
        instruction_sheet: &InstructionSheet,
        stack_snapshot: &StackSnapshot<'eval>,
        parent: &Trace,
    ) -> Avec<Trace<'eval>> {
        let history = exec_debug(compile_time.upcast(), instruction_sheet, stack_snapshot);
        Arc::new(self.proc_stmts_traces(parent.id, parent.indent + 2, stmts, text, &history))
    }

    pub(super) fn proc_branch_lines(
        &self,
        text: &Text,
        indent: Indent,
        branch: &ProcConditionBranch,
        history: &Arc<History<'eval>>,
    ) -> Vec<LineProps<'eval>> {
        vec![LineProps {
            indent,
            tokens: self.proc_branch_tokens(text, indent, branch, history),
            idx: 0,
        }]
    }

    pub(super) fn proc_branch_tokens(
        &self,
        text: &Text,
        indent: Indent,
        branch: &ProcConditionBranch,
        history: &Arc<History<'eval>>,
    ) -> Vec<TokenProps<'eval>> {
        let mut tokens = Vec::new();
        match branch.variant {
            ProcConditionBranchVariant::If { ref condition } => {
                tokens.push(keyword!("if "));
                tokens.extend(self.eager_expr_tokens(
                    condition,
                    text,
                    history,
                    ExprTokenConfig::branch(),
                ));
                tokens.push(special!(":"))
            }
            ProcConditionBranchVariant::Elif { ref condition } => {
                tokens.push(keyword!("elif "));
                tokens.extend(self.eager_expr_tokens(
                    condition,
                    text,
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
