use super::*;

impl HuskyTraceTime {
    pub(crate) fn proc_stmt_figure(
        &self,
        stmt: &ProcStmt,
        history: &History<'static>,
    ) -> FigureCanvasData {
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
                    FigureCanvasData::void()
                }
            }
            ProcStmtVariant::Return { ref result, .. } => self.eager_expr_figure(result, history),
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
                    FigureCanvasData::void()
                }
            }
            ProcStmtVariant::Break => FigureCanvasData::void(),
            ProcStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => todo!(),
        }
    }

    pub fn loop_frame_mutations_figure(
        &self,
        loop_trace_id: TraceId,
        frame_mutations: &[MutationData],
        frame_stack_snapshot: &StackSnapshot,
    ) -> FigureCanvasData {
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
                    .map(|(idx, mutation_data)| {
                        if let Some(frame_mutation) =
                            frame_mutations.iter().find(|frame_mutation| {
                                frame_mutation.varidx() == mutation_data.varidx()
                            })
                        {
                            self.mutation_figure(idx, mutation_data)
                        } else {
                            MutationFigureData {
                                name: match mutation_data.kind {
                                    MutationDataVariant::Exec => panic!(),
                                    MutationDataVariant::Block { stack_idx, varname } => {
                                        varname.as_str().to_string()
                                    }
                                },
                                before: None,
                                after: FigureCanvasData::new_specific(
                                    self.visualize_temp_value(
                                        &stack_snapshot[mutation_data.varidx()].snapshot(),
                                        mutation_data.ty,
                                        mutation_data.file,
                                        mutation_data.range,
                                    )
                                    .unwrap(),
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
        FigureCanvasData::Mutations { mutations }
    }

    pub fn mutations_figure(&self, mutations: &[MutationData<'static>]) -> FigureCanvasData {
        FigureCanvasData::Mutations {
            mutations: mutations
                .iter()
                .enumerate()
                .map(|(idx, mutation)| self.mutation_figure(idx, mutation))
                .collect(),
        }
    }

    fn mutation_figure(
        &self,
        idx: usize,
        mutation_data: &MutationData<'static>,
    ) -> MutationFigureData {
        let sample_id = self.restriction.opt_sample_id().unwrap();
        MutationFigureData {
            name: match mutation_data.kind {
                MutationDataVariant::Exec => {
                    let text = self
                        .runtime()
                        .compile_time()
                        .text(mutation_data.file)
                        .unwrap();
                    text.ranged(mutation_data.range)
                }
                MutationDataVariant::Block { varname, .. } => varname.as_str().to_string(),
            },
            before: if let Some(before) = mutation_data.before.as_ref() {
                Some(FigureCanvasData::new_specific(
                    self.visualize_temp_value(
                        before,
                        mutation_data.ty,
                        mutation_data.file,
                        mutation_data.range,
                    )
                    .unwrap(),
                ))
            } else {
                None
            },
            after: FigureCanvasData::new_specific(
                self.visualize_temp_value(
                    &mutation_data.after,
                    mutation_data.ty,
                    mutation_data.file,
                    mutation_data.range,
                )
                .unwrap(),
            )
            .into(),
            idx,
        }
    }
}
