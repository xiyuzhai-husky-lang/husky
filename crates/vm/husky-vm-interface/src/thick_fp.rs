use std::marker::PhantomData;

use crate::*;

#[derive(Clone, Copy)]
pub struct __ThickFp<F: for<'eval> BaseThinFp> {
    needs_eval_context: bool,
    fp: *const (),
    phantom: PhantomData<F>,
}

impl<F: for<'eval> BaseThinFp> __ThickFp<F> {
    pub fn call1<'eval, A1, Output>(self, __ctx: &dyn __EvalContext<'eval>, a1: A1) -> Output
    where
        F: Fn(A1) -> Output,
    {
        todo!()
    }
}

impl<'eval, F: BaseThinFp> __StaticInfo for __ThickFp<F>
where
    F::__StaticSelf: BaseThinFp,
{
    type __StaticSelf = __ThickFp<F::__StaticSelf>;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        todo!()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf
    where
        Self: Sized,
    {
        todo!()
    }
}

impl<'eval, F: BaseThinFp> __WithEvalLifetime<'eval> for __ThickFp<F> {
    type __SelfWithEvalLifetime = Self;
}

impl<F: for<'eval> BaseThinFp> __Any for __ThickFp<F> where F::__StaticSelf: for<'eval> BaseThinFp {}
