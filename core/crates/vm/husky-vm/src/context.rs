use std::marker::PhantomData;

pub struct __EvalContext<'eval> {
    phantom: PhantomData<&'eval ()>,
}
