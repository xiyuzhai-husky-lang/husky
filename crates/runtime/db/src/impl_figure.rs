mod figure_focus;
mod impl_figure_control;

pub use figure_focus::*;
pub use impl_figure_control::*;

use crate::*;
use compile_time_db::*;
use feature::{FeatureExpr, FeatureExprVariant, FeatureStmt, FeatureStmtVariant};
use map_collect::MapCollect;
use semantics_eager::{
    EagerExpr, EagerExprVariant, FuncStmt, FuncStmtVariant, ProcStmt, ProcStmtVariant,
};
use text::TextQueryGroup;
use trace::MutationFigureProps;
use vm::{History, HistoryEntry, MutationData, MutationDataKind, StackSnapshot};

impl HuskyLangRuntime {
    pub fn figure(&self, trace_id: TraceId, focus: &Focus) -> FigureProps {
        let trace = self.trace(trace_id);
        match trace.variant {
            TraceVariant::Main(_) => FigureProps::void(),
            TraceVariant::FeatureStmt(ref stmt) => self.feature_stmt_figure(stmt, focus),
            TraceVariant::FeatureBranch(_) => FigureProps::void(),
            TraceVariant::FeatureExpr(ref expr) => self.feature_expr_figure(expr, focus),
            TraceVariant::FeatureCallInput { ref input, .. } => {
                self.feature_expr_figure(input, focus)
            }
            TraceVariant::FuncStmt {
                ref stmt,
                ref history,
            } => self.func_stmt_figure(stmt, history),
            TraceVariant::ProcStmt {
                ref stmt,
                ref history,
            } => self.proc_stmt_figure(stmt, history),
            TraceVariant::EagerExpr {
                ref expr,
                ref history,
            } => self.eager_expr_figure(expr, history),
            TraceVariant::CallHead {
                ref entity,
                ref tokens,
            } => FigureProps::void(),
            TraceVariant::LoopFrame {
                ref loop_frame_data,
                ..
            } => self.loop_frame_mutations_figure(
                trace.parent.unwrap(),
                &loop_frame_data.mutations,
                &loop_frame_data.stack_snapshot,
            ),
            TraceVariant::ProcBranch {
                ref stmt,
                branch_idx,
                ref history,
                ..
            } => match history.get(stmt) {
                Some(HistoryEntry::ConditionFlow {
                    opt_branch_entered: branch_entered,
                    mutations,
                    ..
                }) => {
                    if *branch_entered == Some(branch_idx) {
                        self.mutations_figure(mutations)
                    } else {
                        FigureProps::void()
                    }
                }
                None => FigureProps::void(),
                _ => panic!(),
            },
        }
    }

    fn feature_stmt_figure(&self, stmt: &FeatureStmt, focus: &Focus) -> FigureProps {
        match stmt.variant {
            FeatureStmtVariant::Init { varname, ref value } => {
                self.feature_expr_figure(value, focus)
            }
            FeatureStmtVariant::Assert { .. } => FigureProps::void(),
            FeatureStmtVariant::Return { ref result } => self.feature_expr_figure(result, focus),
            FeatureStmtVariant::ConditionFlow { ref branches } => FigureProps::void(),
        }
    }

    fn feature_expr_figure(&self, expr: &FeatureExpr, focus: &Focus) -> FigureProps {
        match focus.opt_input_id {
            Some(input_id) => {
                if let Ok(value) = self.eval_feature_expr(expr, input_id) {
                    let visualizer = self.visualizer(expr.expr.ty());
                    let visual_props = visualizer.visualize(self, value.any_ref());
                    FigureProps::new_specific(visual_props)
                } else {
                    FigureProps::void()
                }
            }
            None => todo!(),
        }
    }

    fn func_stmt_figure(&self, stmt: &FuncStmt, history: &History) -> FigureProps {
        match stmt.variant {
            FuncStmtVariant::Init {
                varname,
                ref initial_value,
            } => todo!(),
            FuncStmtVariant::Assert { ref condition } => todo!(),
            FuncStmtVariant::Return { ref result } => todo!(),
            FuncStmtVariant::ConditionFlow { ref branches } => todo!(),
            FuncStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => todo!(),
            FuncStmtVariant::ReturnXml { ref xml_expr } => todo!(),
        }
    }

    fn proc_stmt_figure(&self, stmt: &ProcStmt, history: &History) -> FigureProps {
        match stmt.variant {
            ProcStmtVariant::Init {
                varname,
                ref initial_value,
                init_kind,
            } => self.eager_expr_figure(initial_value, history),
            ProcStmtVariant::Assert { ref condition } => todo!(),
            ProcStmtVariant::Execute { ref expr } => {
                if let Some(entry) = history.get(expr) {
                    match entry {
                        HistoryEntry::Exec { ref mutations } => self.mutations_figure(mutations),
                        _ => {
                            p!(entry);
                            panic!("wrong kind of entry")
                        }
                    }
                } else {
                    FigureProps::void()
                }
            }
            ProcStmtVariant::Return { ref result } => self.eager_expr_figure(result, history),
            ProcStmtVariant::ConditionFlow { ref branches } => todo!(),
            ProcStmtVariant::Loop {
                ref loop_variant,
                ref stmts,
            } => {
                if let Some(entry) = history.get(stmt) {
                    match entry {
                        HistoryEntry::Loop { ref mutations, .. } => {
                            self.mutations_figure(mutations)
                        }
                        _ => panic!(),
                    }
                } else {
                    FigureProps::void()
                }
            }
            ProcStmtVariant::Break => FigureProps::void(),
            ProcStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => todo!(),
        }
    }

    fn eager_expr_figure(&self, expr: &EagerExpr, history: &History) -> FigureProps {
        let visualizer = self.visualizer(expr.ty);
        if let Some(entry) = history.get(expr) {
            match entry {
                HistoryEntry::PureExpr { output } => match output {
                    Ok(output) => {
                        let visual_props = visualizer.visualize(self, output.any_ref());
                        FigureProps::new_specific(visual_props)
                    }
                    Err(e) => FigureProps::void(),
                },
                HistoryEntry::Exec { .. } => todo!(),
                HistoryEntry::Loop { .. } => panic!(),
                HistoryEntry::ConditionFlow {
                    opt_branch_entered: enter,
                    ..
                } => todo!(),
                HistoryEntry::Break => todo!(),
                HistoryEntry::PatternMatching { .. } => todo!(),
            }
        } else {
            FigureProps::void()
        }
    }

    pub fn mutations_figure(&self, mutations: &[MutationData]) -> FigureProps {
        FigureProps::Mutations {
            mutations: mutations
                .iter()
                .enumerate()
                .map(|(i, mutation)| {
                    MutationFigureProps::new(
                        self,
                        &self.compile_time().text(mutation.file).unwrap(),
                        &self.visualizer(mutation.ty),
                        mutation,
                        i,
                    )
                })
                .collect(),
        }
    }

    pub fn loop_frame_mutations_figure(
        &self,
        loop_trace_id: TraceId,
        frame_mutations: &[MutationData],
        frame_stack_snapshot: &StackSnapshot,
    ) -> FigureProps {
        let loop_trace = self.trace(loop_trace_id);
        let mutations = match loop_trace.variant {
            TraceVariant::ProcStmt {
                ref stmt,
                ref history,
            } => match history.get(stmt).unwrap() {
                HistoryEntry::Loop {
                    loop_kind,
                    control,
                    stack_snapshot,
                    body_instruction_sheet: body,
                    mutations,
                } => mutations
                    .iter()
                    .enumerate()
                    .map(|(idx, mutation)| {
                        if let Some(frame_mutation) = frame_mutations
                            .iter()
                            .find(|frame_mutation| frame_mutation.varidx() == mutation.varidx())
                        {
                            MutationFigureProps::new(
                                self,
                                &self.compile_time().text(frame_mutation.file).unwrap(),
                                &self.visualizer(frame_mutation.ty),
                                frame_mutation,
                                idx,
                            )
                        } else {
                            MutationFigureProps {
                                name: match mutation.kind {
                                    MutationDataKind::Exec { range } => panic!(),
                                    MutationDataKind::Block { stack_idx, varname } => {
                                        varname.as_str().to_string()
                                    }
                                },
                                before: None,
                                after: FigureProps::new_specific(
                                    self.visualizer(mutation.ty).visualize(
                                        self,
                                        frame_stack_snapshot[mutation.varidx()].any_ref(),
                                    ),
                                ),
                                idx,
                            }
                        }
                    })
                    .collect(),
                _ => panic!(),
            },
            _ => panic!(),
        };
        FigureProps::Mutations { mutations }
    }
}
