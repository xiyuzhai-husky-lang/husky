use super::*;
use husky_task_prelude::{all_ritchies, DevEvalContext};
use smallvec::SmallVec;

pub use husky_standard_value::{value_conversion, FromValue, IntoValue, Value};

pub type LinkageImplValueResult = Result<Value, /* ad hoc */ ()>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkageImpl<Pedestal>
where
    Pedestal: Copy + 'static,
{
    RitchieFn {
        fn_wrapper: fn(
            DevEvalContext<LinkageImpl<Pedestal>>,
            &[ValArgumentReprInterface],
        ) -> LinkageImplValueResult,
        fn_pointer: fn(),
    },
    RitchieGn {
        generic_pedestal: fn(Pedestal) -> Pedestal,
        gn_generic_wrapper: fn(Pedestal, &[ValArgumentReprInterface]) -> LinkageImplValueResult,
        gn_specific_wrapper: fn(
            DevEvalContext<LinkageImpl<Pedestal>>,
            &[ValArgumentReprInterface],
            Value,
        ) -> LinkageImplValueResult,
    },
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
        val_repr: ValReprInterface,
        ctx: DevEvalContext<Self>,
        val_argument_reprs: &[ValArgumentReprInterface],
    ) -> LinkageImplValueResult {
        match self {
            LinkageImpl::RitchieFn { fn_wrapper, .. } => fn_wrapper(ctx, val_argument_reprs),
            LinkageImpl::RitchieGn {
                generic_pedestal,
                gn_generic_wrapper,
                gn_specific_wrapper,
            } => {
                let value_at_generic_pedestal = ctx.eval_value_at_generic_pedestal(
                    val_repr,
                    generic_pedestal(ctx.pedestal()),
                    gn_generic_wrapper,
                    val_argument_reprs,
                );
                todo!()
            }
        }
    }
}

pub struct FnLinkageImplSource<Pedestal, T>(pub std::marker::PhantomData<Pedestal>, pub T);

all_ritchies! {impl_is_fn_linkage_impl_source}

pub trait IsGnItem {
    type Pedestal;

    fn generic_pedestal(specific_pedestal: Self::Pedestal) -> Self::Pedestal;
}

#[macro_export]
macro_rules! gn_linkage_impl {
    ($gn_item: ty) => {{
        /// generic_pedestal is a pedestal that is not closed (minimal)
        ///
        /// it's the counterpart of generic point in algebraic geometry
        fn gn_generic_wrapper(
            generic_pedestal: __Pedestal,
            val_argument_reprs: &[__ValArgumentReprInterface],
        ) -> __ValueResult {
            todo!("gn_generic_wrapper");
        }
        fn gn_specific_wrapper(
            ctx: __DevEvalContext,
            val_argument_reprs: &[__ValArgumentReprInterface],
            value_at_generic_pedestal: __Value,
        ) -> __ValueResult {
            todo!("gn_specific_wrapper");
        }
        __LinkageImpl::RitchieGn {
            // ad hoc
            generic_pedestal: <$gn_item as __IsGnItem>::generic_pedestal,
            gn_generic_wrapper,
            gn_specific_wrapper,
        }
    }};
}
