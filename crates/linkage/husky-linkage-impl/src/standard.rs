use super::*;
use husky_task_prelude::{all_ritchies, DevEvalContext};
use smallvec::SmallVec;

pub use husky_standard_value::{value_conversion, FromValue, IntoValue};

pub type FnArguments = SmallVec<[Value; 4]>;
// ad hoc
pub type GnArguments = SmallVec<[Value; 4]>;
pub type Value = husky_standard_value::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkageImpl<BasePoint>
where
    BasePoint: Copy + 'static,
{
    RitchieFn {
        fn_wrapper: fn(DevEvalContext<BasePoint>, FnArguments) -> Value,
        fn_pointer: fn(),
    },
}

impl<BasePoint> IsLinkageImpl for LinkageImpl<BasePoint>
where
    BasePoint: Copy + 'static,
{
    type BasePoint = BasePoint;
    type Value = Value;
    type FnArguments = FnArguments;
    type GnArguments = GnArguments;

    fn eval_fn(self, ctx: DevEvalContext<BasePoint>, arguments: FnArguments) -> Self::Value {
        match self {
            LinkageImpl::RitchieFn { fn_wrapper, .. } => fn_wrapper(ctx, arguments),
        }
    }

    fn eval_gn(self) -> Self::Value {
        todo!()
    }
}

pub struct LinkageImplSource<BasePoint, T>(pub std::marker::PhantomData<BasePoint>, pub T);

all_ritchies! {impl_into_linkage_impl}
