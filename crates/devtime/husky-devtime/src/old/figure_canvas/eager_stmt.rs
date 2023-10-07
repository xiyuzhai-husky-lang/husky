use husky_syn_expr::SynStmtIdx;

use super::*;

impl<Task: IsTask> Devtime<Task> {
    pub(crate) fn proc_stmt_figure(
        &self,
        stmt: SynStmtIdx,
        history: &History,
    ) -> SpecificFigureCanvasData {
        todo!()
        // match stmt.variant {
        //     HirEagerStmt::Init {
        //         ref initial_value, ..
        //     } => self.eager_expr_figure(initial_value, history),
        //     HirEagerStmt::Assert { .. } => todo!(),
        //     HirEagerStmt::Execute { ref expr } => {
        //         if let Some(entry) = history.get(expr) {
        //             match entry {
        //                 HistoryEntry::Exec { ref mutations } => self.mutations_figure(mutations),
        //                 _ => {
        //                     p!(entry);
        //                     panic!("wrong kind of entry")
        //                 }
        //             }
        //         } else {
        //             Default::default()
        //         }
        //     }
        //     HirEagerStmt::Return { ref result, .. } => self.eager_expr_figure(result, history),
        //     HirEagerStmt::ConditionFlow { .. } => todo!(),
        //     HirEagerStmt::Loop { .. } => {
        //         if let Some(entry) = history.get(stmt) {
        //             match entry {
        //                 HistoryEntry::Loop { ref mutations, .. } => {
        //                     self.mutations_figure(mutations)
        //                 }
        //                 _ => panic!(),
        //             }
        //         } else {
        //             Default::default()
        //         }
        //     }
        //     HirEagerStmt::Break => Default::default(),
        //     HirEagerStmt::Match { .. } => todo!(),
        // }
    }

    pub fn loop_frame_mutations_figure(
        &self,
        loop_trace_id: TraceId,
        frame_mutations: &[MutationData],
    ) -> SpecificFigureCanvasData {
        todo!()
        // let loop_trace = self.trace(loop_trace_id);
        // let mutations = match loop_trace.variant {
        //     TraceVariant::EagerStmt {
        //         stmt, ref history, ..
        //     } => match history.get(stmt).unwrap() {
        //         HistoryEntry::Loop {
        //             stack_snapshot,
        //             mutations,
        //             ..
        //         } => mutations
        //             .iter()
        //             .enumerate()
        //             .map(|(idx, mutation_data)| {
        //                 if frame_mutations
        //                     .iter()
        //                     .find(|frame_mutation| {
        //                         frame_mutation.varidx() == mutation_data.varidx()
        //                     })
        //                     .is_some()
        //                 {
        //                     self.mutation_figure(idx, mutation_data)
        //                 } else {
        //                     MutationFigureData {
        //                         name: match mutation_data.kind {
        //                             MutationDataVariant::Exec => panic!(),
        //                             MutationDataVariant::Block { varname, .. } => {
        //                                 varname.as_str().to_string()
        //                             }
        //                         },
        //                         before: None,
        //                         after: FigureCanvasAtom::new(
        //                             self.visualize_temp_value(
        //                                 &stack_snapshot[mutation_data.varidx()].snapshot(),
        //                                 mutation_data.ty,
        //                                 mutation_data.file,
        //                                 mutation_data.range,
        //                             )
        //                             .unwrap(),
        //                         ),
        //                         idx,
        //                     }
        //                 }
        //             })
        //             .collect(),
        //         _ => panic!(),
        //     },
        //     _ => panic!(),
        // };
        // SpecificFigureCanvasData::Mutations { mutations }
    }

    pub fn mutations_figure(&self, mutations: &[MutationData]) -> SpecificFigureCanvasData {
        SpecificFigureCanvasData::Mutations {
            mutations: mutations
                .iter()
                .enumerate()
                .map(|(idx, mutation)| self.mutation_figure(idx, mutation))
                .collect(),
        }
    }

    fn mutation_figure(&self, idx: usize, mutation_data: &MutationData) -> MutationFigureData {
        todo!()
        // MutationFigureData {
        //     name: match mutation_data.kind {
        //         MutationDataVariant::Exec => {
        //             let text = self.runtime().text(mutation_data.file).unwrap();
        //             text.ranged(mutation_data.range)
        //         }
        //         MutationDataVariant::Block { varname, .. } => varname.as_str().to_string(),
        //     },
        //     before: if let Some(before) = mutation_data.before.as_ref() {
        //         FigureCanvasAtom::new(
        //             self.visualize_temp_value(
        //                 before,
        //                 mutation_data.ty,
        //                 mutation_data.file,
        //                 mutation_data.range,
        //             )
        //             .unwrap(),
        //         )
        //     } else {
        //         None
        //     },
        //     after: FigureCanvasAtom::new(
        //         self.visualize_temp_value(
        //             &mutation_data.after,
        //             mutation_data.ty,
        //             mutation_data.file,
        //             mutation_data.range,
        //         )
        //         .unwrap(),
        //     ),
        //     idx,
        // }
    }
}
