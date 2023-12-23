use super::*;
use husky_task_prelude::{all_ritchies, DevEvalContext, LinkageImplValueResult};
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
        fn_wrapper: fn(DevEvalContext<Self>, &[ValArgumentReprInterface]) -> Value,
        fn_pointer: fn(),
    },
    RitchieGn,
}

impl<Pedestal> IsLinkageImpl for LinkageImpl<Pedestal>
where
    Pedestal: Copy + 'static,
{
    type Pedestal = Pedestal;
    type Value = Value;
    type Error = ();

    fn eval(
        self,
        ctx: DevEvalContext<Self>,
        arguments: &[ValArgumentReprInterface],
    ) -> LinkageImplValueResult<Self> {
        match self {
            LinkageImpl::RitchieFn { fn_wrapper, .. } => {
                LinkageImplValueResult::<Self>::Ok(fn_wrapper(ctx, arguments))
            }
            LinkageImpl::RitchieGn => todo!(),
        }
    }
}

pub struct FnLinkageImplSource<Pedestal, T>(pub std::marker::PhantomData<Pedestal>, pub T);

all_ritchies! {impl_is_fn_linkage_impl_source}

#[macro_export]
macro_rules! gn_linkage_impl {
    ($gn_item: expr) => {{
        fn gn_wrapper(arguments: GnArguments) -> Value {
            todo!();
        }
        LinkageImpl::RitchieGn
    }};
}
