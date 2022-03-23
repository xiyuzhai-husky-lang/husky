use text::Text;
use vm::{
    exec_debug, exec_loop_debug, BoundaryKind, History, InstructionSheet, LoopFrameSnapshot,
    StackSnapshot, VMControl,
};

use super::{expr::ExprTokenConfig, *};
use crate::*;

impl<'eval> TraceFactory<'eval> {
    fn new_impr_stmt_trace(
        &self,
        parent_id: TraceId,
        indent: Indent,
        stmt: Arc<ImprStmt>,
        text: &Text,
        history: Arc<History<'eval>>,
    ) -> Arc<Trace<'eval>> {
        self.new_trace(
            Some(parent_id),
            indent,
            TraceKind::ImprStmt { stmt, history },
            text,
        )
    }

    pub(super) fn impr_stmt_lines(
        &self,
        stmt: &ImprStmt,
        text: &Text,
        history: &Arc<History<'eval>>,
    ) -> Vec<LineProps<'eval>> {
        vec![LineProps {
            indent: stmt.indent,
            tokens: self.impr_stmt_tokens(stmt, text, history),
            idx: 0,
        }]
    }

    pub(super) fn impr_stmt_tokens(
        &self,
        stmt: &ImprStmt,
        text: &Text,
        history: &Arc<History<'eval>>,
    ) -> Vec<TokenProps<'eval>> {
        match stmt.kind {
            ImprStmtKind::Init {
                varname,
                ref initial_value,
                init_kind,
                varidx,
            } => {
                let mut tokens = vec![keyword!(match init_kind {
                    vm::InitKind::Let => "let ",
                    vm::InitKind::Var => "var ",
                    vm::InitKind::Decl => panic!(),
                })];
                tokens.push(ident!(varname.0));
                tokens.push(special!(" = "));
                tokens.extend(self.eager_expr_tokens(
                    initial_value,
                    text,
                    history,
                    ExprTokenConfig::stmt(),
                ));
                tokens
            }
            ImprStmtKind::Assert { ref condition } => {
                let mut tokens = vec![keyword!("assert ")];
                tokens.extend(self.eager_expr_tokens(
                    condition,
                    text,
                    history,
                    ExprTokenConfig::stmt(),
                ));
                tokens
            }
            ImprStmtKind::Execute { ref expr } => {
                self.eager_expr_tokens(expr, text, history, ExprTokenConfig::exec())
            }
            ImprStmtKind::Return { ref result } => {
                let mut tokens = vec![keyword!("return ")];
                tokens.extend(self.eager_expr_tokens(
                    result,
                    text,
                    history,
                    ExprTokenConfig::stmt(),
                ));
                tokens
            }
            ImprStmtKind::BranchGroup { kind, ref branches } => todo!(),
            ImprStmtKind::Loop {
                ref loop_kind,
                ref stmts,
            } => match loop_kind {
                LoopKind::For {
                    frame_var,
                    ref initial_boundary,
                    ref final_boundary,
                    ..
                } => {
                    let mut tokens = vec![keyword!("for ")];
                    tokens.extend(self.initial_boundary_tokens(initial_boundary, text, history));
                    tokens.push(ident!(frame_var.0));
                    tokens.extend(self.final_boundary_tokens(final_boundary, text, history));
                    tokens.push(special!(":"));
                    tokens
                }
                LoopKind::ForExt {
                    frame_var,
                    ref final_boundary,
                    ..
                } => {
                    let mut tokens = vec![keyword!("forext ")];
                    tokens.push(ident!(frame_var.0));
                    tokens.extend(self.final_boundary_tokens(final_boundary, text, history));
                    tokens.push(special!(":"));
                    tokens
                }
                LoopKind::While { ref condition } => {
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
                LoopKind::DoWhile { condition } => {
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

    pub fn impr_stmts_traces<'a>(
        &'a self,
        parent_id: TraceId,
        indent: Indent,
        stmts: &'a [Arc<ImprStmt>],
        text: &'a Text,
        history: &'a Arc<History<'eval>>,
    ) -> impl Iterator<Item = Arc<Trace<'eval>>> + 'a {
        stmts.iter().map(move |stmt| {
            self.new_impr_stmt_trace(parent_id, indent, stmt.clone(), text, history.clone())
        })
    }

    pub(super) fn loop_subtraces(
        &self,
        parent: &Trace,
        loop_kind: &LoopKind,
        loop_stmt: &Arc<ImprStmt>,
        body_stmts: &Arc<Vec<Arc<ImprStmt>>>,
        text: &Text,
        stack_snapshot: &StackSnapshot<'eval>,
        body_instruction_sheet: &Arc<InstructionSheet>,
    ) -> Arc<Vec<Arc<Trace<'eval>>>> {
        let frames = exec_loop_debug(stack_snapshot, loop_kind.into(), &body_instruction_sheet);
        Arc::new(
            frames
                .into_iter()
                .map(|loop_frame_snapshot| {
                    self.new_trace(
                        Some(parent.id),
                        parent.indent + 2,
                        TraceKind::LoopFrame {
                            loop_stmt: loop_stmt.clone(),
                            body_stmts: body_stmts.clone(),
                            body_instruction_sheet: body_instruction_sheet.clone(),
                            loop_frame_snapshot,
                        },
                        text,
                    )
                })
                .collect(),
        )
    }

    pub(super) fn loop_frame_subtraces(
        &self,
        parent: &Trace,
        loop_frame_snapshot: &LoopFrameSnapshot<'eval>,
        instruction_sheet: &InstructionSheet,
        stmts: &[Arc<ImprStmt>],
        text: &Text,
    ) -> Arc<Vec<Arc<Trace<'eval>>>> {
        let history = exec_debug(&loop_frame_snapshot.stack, instruction_sheet);
        Arc::new(
            self.impr_stmts_traces(parent.id, parent.indent + 2, stmts, text, &history)
                .collect(),
        )
    }

    pub(super) fn loop_frame_lines(
        &self,
        indent: Indent,
        vm_loop_frame: &LoopFrameSnapshot,
    ) -> Vec<LineProps<'eval>> {
        vec![LineProps {
            indent,
            tokens: self.loop_frame_tokens(vm_loop_frame),
            idx: 0,
        }]
    }

    pub(super) fn loop_frame_tokens(
        &self,
        vm_loop_frame: &LoopFrameSnapshot,
    ) -> Vec<TokenProps<'eval>> {
        match vm_loop_frame.kind {
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
}
