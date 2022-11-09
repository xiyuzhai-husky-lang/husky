mod impl_eager_expr;
mod impl_feature_expr;

use super::*;

impl Debugtime {
    pub(crate) fn routine_call_subtraces<A>(
        &mut self,
        parent: &Trace,
        instruction_sheet: &InstructionSheet,
        routine_defn: &Arc<EntityDefn>,
        arguments: &[A],
        argument_trace_gen: impl Fn(
            &mut Self,
            &A,
            &'static str,
        ) -> (TraceId, Option<__VMResult<__Register<'static>>>),
    ) -> Vec<TraceId> {
        if self.state.presentation().opt_sample_id().is_some() {
            // let instruction_sheet: &InstructionSheet = opt_instruction_sheet.as_ref().unwrap();
            let mut subtraces = vec![];
            let mut func_input_values = vec![];
            subtraces.push(self.new_call_head_trace(parent, routine_defn.clone()));
            let argnames: Vec<&'static str> = match routine_defn.variant {
                EntityDefnVariant::Func { ref parameters, .. } => parameters
                    .iter()
                    .map(|param| param.ident().as_str())
                    .collect(),
                EntityDefnVariant::Proc { ref parameters, .. } => parameters
                    .iter()
                    .map(|param| param.ident().as_str())
                    .collect(),
                EntityDefnVariant::Function { ref parameters, .. } => parameters
                    .iter()
                    .map(|param| param.ident().as_str())
                    .collect(),
                EntityDefnVariant::Method { ref parameters, .. } => {
                    let mut argnames: Vec<_> = parameters
                        .iter()
                        .map(|param| param.ident().as_str())
                        .collect();
                    argnames.insert(0, "this");
                    argnames
                }
                _ => panic!(),
            };
            for (i, argument) in arguments.iter().enumerate() {
                let (argument_trace, result) = argument_trace_gen(self, argument, argnames[i]);
                subtraces.push(argument_trace);
                if let Some(result) = result {
                    match result {
                        Ok(value) => func_input_values.push(value),
                        Err(_) => return subtraces,
                    }
                } else {
                    todo!()
                }
            }
            let sample_id = self.state.presentation().opt_sample_id().unwrap();
            let evaluator = self.runtime().evaluator(sample_id);
            let history = exec_debug(
                self.runtime(),
                unsafe { evaluator.some_ctx() },
                instruction_sheet,
                func_input_values.into_iter().map(|value| value).into(),
                self.vm_config(),
            );
            match routine_defn.variant {
                EntityDefnVariant::Func { ref stmts, .. } => {
                    subtraces.extend(self.func_stmts_traces(parent.id(), 4, stmts, &history));
                }
                EntityDefnVariant::Proc { ref stmts, .. } => {
                    subtraces.extend(self.proc_stmts_traces(parent.id(), 4, stmts, &history));
                }
                EntityDefnVariant::Function { .. } => todo!(),
                EntityDefnVariant::Method { ref opt_source, .. } => match opt_source
                    .as_ref()
                    .unwrap()
                {
                    CallFormSource::Func { stmts } => {
                        subtraces.extend(self.func_stmts_traces(parent.id(), 4, stmts, &history));
                    }
                    CallFormSource::Proc { stmts } => {
                        subtraces.extend(self.proc_stmts_traces(parent.id(), 4, stmts, &history));
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            }
            subtraces
        } else {
            vec![]
        }
    }
}
