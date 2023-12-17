use super::*;
use husky_task_prelude::{all_ritchies, DevEvalContext};
use smallvec::SmallVec;

pub use husky_standard_value::{value_conversion, FromValue, IntoValue, Value};

pub type FnArguments = SmallVec<[Value; 4]>;
// ad hoc
pub type GnArguments = SmallVec<[Value; 4]>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkageImpl<BasePoint>
where
    BasePoint: Copy + 'static,
{
    RitchieFn {
        fn_wrapper: fn(DevEvalContext<Self>, FnArguments) -> Value,
        fn_pointer: fn(),
    },
}

impl<BasePoint> IsLinkageImpl for LinkageImpl<BasePoint>
where
    BasePoint: Copy + 'static,
{
    type BasePoint = BasePoint;
    type Value = Value;
    type Error = ();
    type FnArguments = FnArguments;
    type GnArguments = GnArguments;

    fn eval_fn(self, ctx: DevEvalContext<Self>, arguments: FnArguments) -> Self::Value {
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
