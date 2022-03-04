use super::*;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(super) fn eval_routine_call(
        &mut self,
        instrns: &[Instruction],
        maybe_compiled: Option<()>,
        inputs: &[Arc<FeatureExpr>],
    ) -> EvalValue<'eval, 'eval> {
        let values: Vec<StackValue<'eval, 'eval>> = inputs
            .iter()
            .map(|expr| self.eval_feature_expr(expr)?.defined())
            .collect::<VMResult<_>>()?;
        let mut stack = BasicInterpreter::new(values);
        if let Some(compiled) = maybe_compiled {
            todo!()
        } else {
            match stack.exec_all(instrns)? {
                vm::ControlSignal::Normal | vm::ControlSignal::Break => panic!(),
                vm::ControlSignal::Return(value) => Ok(Conditional::Defined(value)),
            }
        }
    }
}
