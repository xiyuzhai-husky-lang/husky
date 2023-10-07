mod eager_expr;
mod eager_stmt;
mod lazy_expr;
mod lazy_stmt;
mod utils;
mod val_repr;

use crate::*;
use husky_val_repr::{ValStmt, ValStmtData};
use husky_vm::{History, HistoryEntry, MutationData, MutationDataVariant};

impl<Task: IsTask> Devtime<Task> {
    pub fn gen_trace_generic_figure(
        &self,
        trace_id: TraceId,
    ) -> Result<GenericFigureCanvasData, (SampleId, __VMError)> {
        let trace = self.trace(trace_id);
        Ok(match trace.variant {
            TraceVariant::Main(..) | TraceVariant::Module { .. } => Default::default(),
            TraceVariant::ValStmt(ref stmt) => self.feature_stmt_generic_figure(stmt)?,
            TraceVariant::ValBranch(_) => Default::default(),
            TraceVariant::EntityVal { .. } => todo!(),
            // self.feature_repr_generic_figure(repr)?,
            TraceVariant::LazyExpr(ref expr) => self.feature_expr_generic_figure(expr)?,
            TraceVariant::ValCallArgument {
                argument: ref input,
                ..
            } => self.feature_expr_generic_figure(input)?,
            TraceVariant::EagerStmt {
                stmt: _,
                history: _,
                ..
            } => todo!(),
            // self.proc_stmt_figure(stmt, history).into(),
            TraceVariant::EagerExpr {
                expr: _,
                history: _,
            } => todo!(),
            // self.eager_expr_figure(expr, history).into(),
            TraceVariant::CallHead { .. } => Default::default(),
            TraceVariant::LoopFrame { .. } => todo!(),
            // self
            //     .loop_frame_mutations_figure(
            //         trace.raw_data.opt_parent_id.unwrap(),
            //         &loop_frame_data.mutations,
            //     )
            //     .into(),
            TraceVariant::EagerBranch { .. } => todo!(),
            // match history.get(stmt) {
            //     Some(HistoryEntry::ControlFlow {
            //         opt_branch_entered: branch_entered,
            //         mutations,
            //         ..
            //     }) => {
            //         if *branch_entered == Some(branch_idx) {
            //             self.mutations_figure(mutations).into()
            //         } else {
            //             FigureCanvasData::void()
            //         }
            //     }
            //     None => Default::default(),
            //     _ => panic!(),
            // },
            TraceVariant::EagerCallArgument { .. } => todo!(),
            //  self.eager_expr_figure(argument, history).into(),
        })
    }

    pub fn gen_trace_specific_figure(
        &self,
        trace_id: TraceId,
    ) -> Result<SpecificFigureCanvasData, (SampleId, __VMError)> {
        todo!()
        // let trace = self.trace(trace_id);
        // Ok(match trace.variant {
        //     TraceVariant::Main(..) | TraceVariant::Module { .. } => Default::default(),
        //     TraceVariant::ValStmt(ref stmt) => self.feature_stmt_specific_figure(stmt)?,
        //     TraceVariant::ValBranch(_) => Default::default(),
        //     TraceVariant::EntityVal { ref repr, .. } => self.feature_repr_specific_figure(repr)?,
        //     TraceVariant::LazyExpr(ref expr) => self.feature_expr_specific_figure(expr)?,
        //     TraceVariant::ValCallArgument { ref argument, .. } => {
        //         self.feature_expr_specific_figure(argument)?
        //     }
        //     TraceVariant::FuncStmt {
        //         ref stmt,
        //         ref history,
        //     } => self.func_stmt_figure(stmt, history),
        //     TraceVariant::EagerStmt {
        //         ref stmt,
        //         ref history,
        //         ..
        //     } => self.proc_stmt_figure(stmt, history).into(),
        //     TraceVariant::EagerExpr {
        //         ref expr,
        //         ref history,
        //     } => self.eager_expr_figure(expr, history).into(),
        //     TraceVariant::CallHead { .. } => Default::default(),
        //     TraceVariant::LoopFrame {
        //         ref loop_frame_data,
        //         ..
        //     } => self
        //         .loop_frame_mutations_figure(
        //             trace.raw_data.opt_parent_id.unwrap(),
        //             &loop_frame_data.mutations,
        //         )
        //         .into(),
        //     TraceVariant::FuncBranch {
        //         ref stmt,
        //         branch_idx,
        //         ref history,
        //         ..
        //     } => match history.get(stmt) {
        //         Some(HistoryEntry::ControlFlow {
        //             opt_branch_entered: branch_entered,
        //             control,
        //             ..
        //         }) => {
        //             if *branch_entered == Some(branch_idx) {
        //                 todo!()
        //                 // self.visualize_control(control)
        //             } else {
        //                 Default::default()
        //             }
        //         }
        //         None => Default::default(),
        //         _ => panic!(),
        //     },
        //     TraceVariant::EagerBranch {
        //         ref stmt,
        //         branch_idx,
        //         ref history,
        //         ..
        //     } => match history.get(stmt) {
        //         Some(HistoryEntry::ControlFlow {
        //             opt_branch_entered: branch_entered,
        //             mutations,
        //             ..
        //         }) => {
        //             if *branch_entered == Some(branch_idx) {
        //                 self.mutations_figure(mutations).into()
        //             } else {
        //                 Default::default()
        //             }
        //         }
        //         None => Default::default(),
        //         _ => panic!(),
        //     },
        //     TraceVariant::EagerCallArgument {
        //         ref argument,
        //         ref history,
        //         ..
        //     } => self.eager_expr_figure(argument, history).into(),
        // })
    }
}
