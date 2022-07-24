use crate::*;

pub struct __Linkage {
    wrapper: fn(Option<&dyn __EvalContext>, &mut [__Register]) -> __Register,
    raw: *const (),
}
