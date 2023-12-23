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
        /// it's the wrapper's responsibility to properly set ctx
        fn_wrapper: fn(
            DevEvalContext<LinkageImpl<Pedestal>>,
            &[ValArgumentReprInterface],
        ) -> LinkageImplValueResult,
        fn_pointer: fn(),
    },
    RitchieGn {
        generic_pedestal: fn(Pedestal) -> Pedestal,
        /// it's the wrapper's responsibility to properly set ctx to that with generic pedestal
        gn_generic_wrapper: fn(
            DevEvalContext<LinkageImpl<Pedestal>>,
            &[ValArgumentReprInterface],
        ) -> LinkageImplValueResult,
        /// no need to set ctx
        gn_specific_wrapper: fn(&[ValArgumentReprInterface], Value) -> LinkageImplValueResult,
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
                let value_at_generic_pedestal = ctx.eval_val_repr_at_generic_pedestal_with(
                    val_repr,
                    generic_pedestal,
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
    type Pedestal: Copy + 'static;

    type ValueAtGenericPedestal;

    fn generic_pedestal(specific_pedestal: Self::Pedestal) -> Self::Pedestal;

    /// compute `generic_pedestal` here for efficiency
    fn train(
        val_argument_reprs: &[ValArgumentReprInterface],
    ) -> Result<Self::ValueAtGenericPedestal, ()>;

    fn eval(
        val_argument_reprs: &[ValArgumentReprInterface],
        value_at_generic_pedestal: &Self::ValueAtGenericPedestal,
    ) -> LinkageImplValueResult;
}

#[macro_export]
macro_rules! gn_linkage_impl {
    ($gn_item: ty) => {{
        /// generic_pedestal is a pedestal that is not closed (minimal)
        ///
        /// it's the counterpart of generic point in algebraic geometry
        fn gn_generic_wrapper(
            ctx: __DevEvalContext,
            val_argument_reprs: &[__ValArgumentReprInterface],
        ) -> __ValueResult {
            __with_dev_eval_context(ctx, || {
                <$gn_item as __IsGnItem>::train(val_argument_reprs);
                todo!("gn_generic_wrapper");
            })
        }
        fn gn_specific_wrapper(
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
