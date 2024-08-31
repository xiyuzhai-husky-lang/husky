use crate::{dev_eval_context, DevEvalContext, *};
use husky_devsoul_interface::devsoul::IsDevsoulInterface;
use husky_item_path_interface::ItemPathIdInterface;
use husky_linket_impl::eval_context::DevEvalContextGuard;
use husky_standard_linket_impl::{
    pedestal::StandardPedestal, StandardKiControlFlow, StandardLinketImpl, StandardTrackedExcepted,
    StandardTrackedExceptedValue, StandardTrackedException,
};
use husky_standard_value::FromValue;
use std::cell::OnceCell;

pub struct StandardDevsoulInterface {}

impl IsDevsoulInterface for StandardDevsoulInterface {
    type LinketImpl = StandardLinketImpl;

    fn try_set_dev_eval_context(ctx: DevEvalContext) -> Result<DevEvalContextGuard, ()> {
        try_set_dev_eval_context(ctx)
    }

    fn dev_eval_context() -> DevEvalContext {
        dev_eval_context()
    }
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
