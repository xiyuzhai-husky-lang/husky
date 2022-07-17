use vm::__EvalContext;

pub fn __eval_entity_feature<'eval, T>(
    __ctx: &__EvalContext<'eval>,
    f: fn(__ctx: &__EvalContext<'eval>) -> T,
) -> &'eval T {
    todo!()
}
