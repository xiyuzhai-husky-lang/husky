pub mod devsoul_interface;
pub mod label;
pub mod pedestal;
pub mod static_var;
pub mod ugly;

use self::pedestal::StandardPedestal;
use husky_devsoul_interface::ki_repr::{
    KiDomainReprInterface, KiReprInterface, KiRuntimeConstantInterface,
};
use husky_linket_impl::standard::StandardLinketImplKiControlFlow;
use husky_standard_value::{ugly::__ValueStands, FromValue};
use serde::{Deserialize, Serialize};
use shifted_unsigned_int::ShiftedU32;
use std::{cell::Cell, convert::Infallible};

pub type DevEvalContext = husky_devsoul_interface::DevEvalContext<
    husky_linket_impl::standard::StandardLinketImpl<StandardPedestal>,
>;

/// this is still subject to parallel testing bug, two different tests might want to use the same context.
///
/// it depends on rust's implementation though.
///
/// a possible fix:
///
thread_local! {
    static DEV_EVAL_CONTEXT: std::cell::Cell<std::option::Option<DevEvalContext>> = Default::default();
}

pub fn dev_eval_context() -> DevEvalContext {
    DEV_EVAL_CONTEXT.get().expect("`DEV_EVAL_CONTEXT` not set")
}

pub(crate) fn set_dev_eval_context(ctx: DevEvalContext) {
    assert!(DEV_EVAL_CONTEXT.get().is_none());
    DEV_EVAL_CONTEXT.set(Some(ctx))
}
pub(crate) fn unset_dev_eval_context() {
    assert!(DEV_EVAL_CONTEXT.get().is_some());
    DEV_EVAL_CONTEXT.set(None)
}
/// but this brings some other problems, needs thinking.
// static mut DEV_EVAL_CONTEXT: std::option::Option<DevEvalContext> = None;

// pub fn dev_eval_context() -> DevEvalContext {
//     unsafe { DEV_EVAL_CONTEXT.expect("`DEV_EVAL_CONTEXT` not set") }
// }
// pub(crate) fn set_dev_eval_context(ctx: DevEvalContext) {
//     unsafe {
//         assert!(DEV_EVAL_CONTEXT.is_none());
//         DEV_EVAL_CONTEXT = Some(ctx);
//     }
// }
// pub(crate) fn unset_dev_eval_context() {
//     unsafe {
//         assert!(DEV_EVAL_CONTEXT.is_some());
//         DEV_EVAL_CONTEXT = None;
//     }
// }

pub fn eval_ki_repr_interface<T>(
    ki_repr: KiReprInterface,
    value_stands: Option<&mut __ValueStands>,
) -> StandardLinketImplKiControlFlow<T>
where
    T: FromValue + 'static,
{
    let value = dev_eval_context().eval_ki_repr_interface(ki_repr)?;
    StandardLinketImplKiControlFlow::Continue(<T as FromValue>::from_value_aux(value, value_stands))
}

pub fn eval_ki_domain_repr_interface(
    ki_domain_repr_interface: KiDomainReprInterface,
) -> StandardLinketImplKiControlFlow<(), Infallible> {
    dev_eval_context().eval_ki_domain_repr_interface(ki_domain_repr_interface)
}

pub fn eval_val_runtime_constant<T>(val_runtime_constant: KiRuntimeConstantInterface) -> T
where
    T: FromValue,
{
    // no need to return a stand, because runtime constant are always solid
    T::from_value_static(dev_eval_context().eval_val_runtime_constant(val_runtime_constant))
}
