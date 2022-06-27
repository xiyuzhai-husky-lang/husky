mod impl_eager_expr;
mod impl_feature_expr;

use word::CustomIdentifier;

use super::*;

impl HuskyTraceTime {
    pub(crate) fn routine_call_subtraces<A>(
        &mut self,
        parent: &Trace,
        instruction_sheet: &InstructionSheet,
        routine_defn: &Arc<EntityDefn>,
        arguments: &[A],
        argument_trace_gen: impl Fn(&mut Self, &A, CustomIdentifier) -> (TraceId, EvalResult),
    ) -> Vec<TraceId> {
        if let Some(sample_id) = self.attention.opt_sample_id() {
            // let instruction_sheet: &InstructionSheet = opt_instruction_sheet.as_ref().unwrap();
            let mut subtraces = vec![];
            let mut func_input_values = vec![];
            subtraces.push(self.new_call_head_trace(parent, routine_defn.clone()));
            let parameters: &[Parameter] = match routine_defn.variant {
                EntityDefnVariant::Func { ref parameters, .. } => parameters,
                EntityDefnVariant::Proc { ref parameters, .. } => parameters,
                _ => panic!(),
            };
            for (i, argument) in arguments.iter().enumerate() {
                let (argument_trace, result) =
                    argument_trace_gen(self, argument, parameters[i].ranged_ident.ident);
                subtraces.push(argument_trace);
                //     self.new_trace(
                //     Some(parent.id()),
                //     4,
                //     TraceVariant::FeatureCallArgument {
                //         argument: argument.clone(),
                //         ident: parameters[i].ranged_ident.ident,
                //     },
                // )
                // self
                // .eval_time_singleton
                // .eval_feature_expr(argument, sample_id)
                match result {
                    Ok(value) => func_input_values.push(value),
                    Err(_) => return subtraces,
                }
            }
            let history = exec_debug(
                self.eval_time().upcast(),
                instruction_sheet,
                func_input_values
                    .into_iter()
                    .map(|value| value.into_stack().unwrap()),
                self.eval_time().verbose(),
            );
            match routine_defn.variant {
                EntityDefnVariant::Func { ref stmts, .. } => {
                    subtraces.extend(self.func_stmts_traces(parent.id(), 4, stmts, &history));
                }
                EntityDefnVariant::Proc { ref stmts, .. } => {
                    subtraces.extend(self.proc_stmts_traces(parent.id(), 4, stmts, &history));
                }
                _ => panic!(),
            }
            subtraces
        } else {
            vec![]
        }
    }
}
