pub mod devsoul_interface;
pub mod label;
#[cfg(feature = "ugly")]
pub mod ugly;

use husky_ki_repr_interface::{KiDomainReprInterface, KiReprInterface, KiRuntimeConstantInterface};
use husky_standard_linket_impl::{StandardKiControlFlow, StandardLinketImpl, ValueStands};
use husky_standard_value::FromValue;
use serde::{Deserialize, Serialize};
use shifted_unsigned_int::ShiftedU32;
use std::{cell::Cell, convert::Infallible};

pub type DevEvalContext = husky_linket_impl::eval_context::DevEvalContext<StandardLinketImpl>;

/// this wll be safe because only one runtime have access
///
/// I fear that some usage of runtime in testing will break this, see `dev_runtimes_should_panic`
///
/// If so, put some measures to ensure only one runtime exists at the same time.
static mut DEV_EVAL_CONTEXT: std::option::Option<DevEvalContext> = None;

pub fn dev_eval_context() -> DevEvalContext {
    unsafe { DEV_EVAL_CONTEXT.expect("`DEV_EVAL_CONTEXT` not set") }
}

pub(crate) unsafe fn set_dev_eval_context(ctx: DevEvalContext) {
    assert!(
        DEV_EVAL_CONTEXT.is_none(),
        "current DEV_EVAL_CONTEXT = {:?}, but new ctx = {:?}",
        unsafe { DEV_EVAL_CONTEXT },
        ctx
    );
    DEV_EVAL_CONTEXT = Some(ctx)
}
pub(crate) unsafe fn unset_dev_eval_context() {
    assert!(DEV_EVAL_CONTEXT.is_some());
    DEV_EVAL_CONTEXT = None
}

pub fn eval_ki_repr_interface<T>(
    ki_repr: KiReprInterface,
    value_stands: Option<&mut ValueStands>,
) -> StandardKiControlFlow<T>
where
    T: FromValue + 'static,
{
    let value = dev_eval_context().eval_ki_repr_interface(ki_repr)?;
    StandardKiControlFlow::Continue(<T as FromValue>::from_value_aux(value, value_stands))
}

pub fn eval_ki_domain_repr_interface(
    ki_domain_repr_interface: KiDomainReprInterface,
) -> StandardKiControlFlow<(), Infallible> {
    dev_eval_context().eval_ki_domain_repr_interface(ki_domain_repr_interface)
}

pub fn eval_val_runtime_constant<T>(val_runtime_constant: KiRuntimeConstantInterface) -> T
where
    T: FromValue,
{
    // no need to return a stand, because runtime constant are always solid
    T::from_value_static(dev_eval_context().eval_val_runtime_constant(val_runtime_constant))
}
