use std::marker::PhantomData;

use crate::*;

#[derive(Clone, Copy)]
pub struct ThickFp<F: for<'eval> __BaseThinFp> {
    needs_eval_context: bool,
    fp: *const (),
    phantom: PhantomData<F>,
}

impl<F: for<'eval> __BaseThinFp> ThickFp<F> {
    pub fn call1<'eval, A1, Output>(self, a1: A1, __ctx: &dyn __EvalContext<'eval>) -> Output
    where
        A1: __StaticInfo,
        F: Fn(A1::__StaticSelf) -> Output,
    {
        todo!()
    }
}

impl<'eval, F: __BaseThinFp> __StaticInfo for ThickFp<F>
where
    F::__StaticSelf: __BaseThinFp,
{
    type __StaticSelf = ThickFp<F::__StaticSelf>;

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
