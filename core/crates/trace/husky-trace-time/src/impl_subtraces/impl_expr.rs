mod impl_eager_expr;
mod impl_feature_expr;

use std::borrow::Cow;

use word::CustomIdentifier;

use super::*;

impl HuskyTraceTime {
    pub(crate) fn routine_call_subtraces<A>(
        &mut self,
        parent: &Trace,
        instruction_sheet: &InstructionSheet,
        routine_defn: &Arc<EntityDefn>,
        arguments: &[A],
        argument_trace_gen: impl Fn(&mut Self, &A, &'static str) -> (TraceId, EvalResult),
    ) -> Vec<TraceId> {
        if let Some(sample_id) = self.attention.opt_sample_id() {
            // let instruction_sheet: &InstructionSheet = opt_instruction_sheet.as_ref().unwrap();
            let mut subtraces = vec![];
            let mut func_input_values = vec![];
            subtraces.push(self.new_call_head_trace(parent, routine_defn.clone()));
            let argnames: Vec<&'static str> = match routine_defn.variant {
                EntityDefnVariant::Func { ref parameters, .. } => parameters
                    .iter()
                    .map(|param| param.ranged_ident.ident.0)
                    .collect(),
                EntityDefnVariant::Proc { ref parameters, .. } => parameters
                    .iter()
                    .map(|param| param.ranged_ident.ident.0)
                    .collect(),
                EntityDefnVariant::Function { ref parameters, .. } => parameters
                    .iter()
                    .map(|param| param.ranged_ident.ident.0)
                    .collect(),
                EntityDefnVariant::Method { ref parameters, .. } => {
                    let mut argnames: Vec<_> = parameters
                        .iter()
                        .map(|param| param.ranged_ident.ident.0)
                        .collect();
                    argnames.insert(0, "this");
                    argnames
                }
                _ => panic!(),
            };
            for (i, argument) in arguments.iter().enumerate() {
                let (argument_trace, result) = argument_trace_gen(self, argument, argnames[i]);
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
                EntityDefnVariant::Function { .. } => todo!(),
                EntityDefnVariant::Method {
                    spatial_parameters: ref generic_parameters,
                    this_liason: this_contract,
                    ref parameters,
                    output_ty,
                    output_liason,
                    ..
                } => todo!(),
                _ => panic!(),
            }
            subtraces
        } else {
            vec![]
        }
    }
}
