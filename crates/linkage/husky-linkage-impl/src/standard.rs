use super::*;
use husky_task_prelude::{all_ritchies, DevEvalContext};
use smallvec::SmallVec;

pub use husky_standard_value::{value_conversion, FromValue, IntoValue, Value};

pub type FnArguments = SmallVec<[Value; 4]>;
// ad hoc
pub type GnArguments = SmallVec<[Value; 4]>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkageImpl<Pedestal>
where
    Pedestal: Copy + 'static,
{
    RitchieFn {
        fn_wrapper: fn(DevEvalContext<Self>, FnArguments) -> Value,
        fn_pointer: fn(),
    },
}

impl<Pedestal> IsLinkageImpl for LinkageImpl<Pedestal>
where
    Pedestal: Copy + 'static,
{
    type Pedestal = Pedestal;
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

pub struct LinkageImplSource<Pedestal, T>(pub std::marker::PhantomData<Pedestal>, pub T);

all_ritchies! {impl_into_linkage_impl}
