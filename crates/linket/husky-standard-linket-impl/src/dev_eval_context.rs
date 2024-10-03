use crate::*;
use husky_ki_repr_interface::{KiDomainReprInterface, KiReprInterface, KiRuntimeComptermInterface};
use husky_linket_impl::dev_eval_context::DevEvalContextGuard;
use husky_standard_value::{slush::SlushValues, FromValue};
use serde::{Deserialize, Serialize};
use std::{cell::Cell, convert::Infallible};

pub type DevEvalContext = husky_linket_impl::dev_eval_context::DevEvalContext<StandardLinketImpl>;

/// this wll be safe because only one runtime have access
///
/// I fear that some usage of runtime in testing will break this, see `dev_runtimes_should_panic`
///
/// If so, put some measures to ensure only one runtime exists at the same time.
static mut DEV_EVAL_CONTEXT: std::option::Option<DevEvalContext> = None;

/// this mutex makes set and unset operations atomic
static DEV_EVAL_CONTEXT_MU: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub fn dev_eval_context() -> DevEvalContext {
    unsafe { DEV_EVAL_CONTEXT.expect("`DEV_EVAL_CONTEXT` not set") }
}

pub(crate) fn try_set_dev_eval_context(ctx: DevEvalContext) -> Result<DevEvalContextGuard, ()> {
    let guard = DEV_EVAL_CONTEXT_MU.lock().unwrap();
    unsafe {
        if DEV_EVAL_CONTEXT.is_some() {
            return Err(());
        }
        DEV_EVAL_CONTEXT = Some(ctx);
    }
    Ok(DevEvalContextGuard::new(unset_dev_eval_context))
}

unsafe fn unset_dev_eval_context() {
    let guard = DEV_EVAL_CONTEXT_MU.lock().unwrap();
    assert!(DEV_EVAL_CONTEXT.is_some());
    DEV_EVAL_CONTEXT = None
}

pub fn eval_ki_repr_interface<T>(
    ki_repr: KiReprInterface,
    slush_values: Option<&mut SlushValues>,
) -> StandardKiControlFlow<T>
where
    T: FromValue + 'static,
{
    let value = dev_eval_context().eval_ki_repr_interface(ki_repr)?;
    StandardKiControlFlow::Continue(<T as FromValue>::from_value_aux(value, slush_values))
}

pub fn eval_ki_domain_repr_interface(
    ki_domain_repr_interface: KiDomainReprInterface,
) -> StandardKiControlFlow<(), Infallible> {
    dev_eval_context().eval_ki_domain_repr_interface(ki_domain_repr_interface)
}

pub fn eval_ki_runtime_compterm<T>(ki_runtime_compterm: KiRuntimeComptermInterface) -> T
where
    T: FromValue,
{
    // no need to return a stand, because runtime constant are always solid
    T::from_value_static(dev_eval_context().eval_ki_runtime_compterm(ki_runtime_compterm))
}

pub fn eval_eager_val_with<T>(
    item_path_id_interface: ItemPathIdInterface,
    pedestal: StandardPedestal,
    f: fn() -> StandardKiControlFlow,
) -> T
where
    T: FromValue,
{
    T::from_value_static(dev_eval_context().eval_eager_val_with(
        item_path_id_interface,
        pedestal,
        f,
    ))
}

pub fn eval_lazy_val<T>(
    item_path_id_interface: ItemPathIdInterface,
    pedestal: StandardPedestal,
) -> T
where
    T: FromValue,
{
    T::from_value_static(dev_eval_context().eval_lazy_val(item_path_id_interface, pedestal))
}

pub fn eval_generic_gn_with<T>(
    ki_repr_interface: KiReprInterface,
    pedestal: StandardPedestal,
    f: impl FnOnce() -> StandardKiControlFlow,
) -> StandardKiControlFlow<T>
where
    T: FromValue,
{
    dev_eval_context()
        .eval_generic_gn_with(ki_repr_interface, pedestal, f)
        .map(T::from_value_static)
}

/// currently, it's intentional that `__Self` must be sized
/// todo: generalize this to ?Sized
pub fn eval_memo_field_with<__Self, T>(
    item_path_id_interface: ItemPathIdInterface,
    __self: &'static __Self,
    f: fn(&'static __Self) -> StandardKiControlFlow,
) -> T
where
    T: FromValue,
{
    T::from_value_static(dev_eval_context().eval_memo_field_with(item_path_id_interface, __self, f))
}
