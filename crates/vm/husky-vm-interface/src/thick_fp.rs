use std::{ffi::c_void, marker::PhantomData};

use crate::*;

#[derive(Clone, Copy)]
pub struct ThickFp<F: __BaseThinFp> {
    needs_eval_context: bool,
    fp: *const c_void,
    phantom: PhantomData<F>,
}

impl<F: __BaseThinFp> PartialEq for ThickFp<F> {
    fn eq(&self, other: &Self) -> bool {
        self.needs_eval_context == other.needs_eval_context && self.fp == other.fp
    }
}

impl<F: __BaseThinFp> Eq for ThickFp<F> {}

impl<F> ThickFp<F>
where
    F: __BaseThinFp,
{
    pub(crate) const fn new(needs_eval_context: bool, fp: *const c_void) -> Self {
        Self {
            needs_eval_context,
            fp,
            phantom: PhantomData,
        }
    }

    pub fn __new_base(f: F) -> Self {
        Self {
            needs_eval_context: false,
            fp: f.__to_void_pointer(),
            phantom: PhantomData,
        }
    }

    pub fn __new_ctx(f: F::__CtxThinFp) -> Self {
        Self {
            needs_eval_context: true,
            fp: f.__to_void_pointer(),
            phantom: PhantomData,
        }
    }

    pub fn call1<'static, A1, Output>(self, a1: A1, __ctx: &dyn __EvalContext) -> Output
    where
        A1: __StaticInfo,
        F: Fn(A1::__StaticSelf) -> Output,
    {
        unsafe {
            match self.needs_eval_context {
                true => {
                    let f: fn(A1, &dyn __EvalContext) -> Output = std::mem::transmute(self.fp);
                    f(a1, __ctx)
                }
                false => {
                    let f: fn(A1) -> Output = std::mem::transmute(self.fp);
                    f(a1)
                }
            }
        }
    }
}

impl<F: __BaseThinFp> __StaticInfo for ThickFp<F>
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
