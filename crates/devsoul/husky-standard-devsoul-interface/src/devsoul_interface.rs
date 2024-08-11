use crate::{dev_eval_context, DevEvalContext, *};
use husky_devsoul_interface::devsoul::IsDevsoulInterface;
use husky_item_path_interface::ItemPathIdInterface;
use husky_standard_linket_impl::{
    pedestal::StandardPedestal, ugly::__Pedestal, StandardLinketImpl,
    StandardLinketImplKiControlFlow, StandardTrackedExcepted, StandardTrackedException,
};
use husky_standard_value::FromValue;
use std::cell::OnceCell;

pub struct StandardDevsoulInterface {}

/// It looks like this definition is not safe.
///
/// However, the program is only going to touch this place in a mutable way, and in a sequential manner.
///
/// Then it will become immutable effectively;

impl IsDevsoulInterface for StandardDevsoulInterface {
    type LinketImpl = StandardLinketImpl;

    fn set_dev_eval_context(ctx: DevEvalContext) {
        set_dev_eval_context(ctx)
    }

    fn unset_dev_eval_context() {
        unset_dev_eval_context()
    }

    fn dev_eval_context() -> DevEvalContext {
        dev_eval_context()
    }
}

pub fn eval_eager_val_with<T>(
    item_path_id_interface: ItemPathIdInterface,
    pedestal: StandardPedestal,
    f: fn() -> StandardLinketImplKiControlFlow,
) -> StandardTrackedExcepted<T>
where
    T: FromValue,
{
    dev_eval_context()
        .eval_eager_val_with(item_path_id_interface, pedestal, f)
        .map(T::from_value_static)
}

pub fn eval_lazy_val<T>(
    item_path_id_interface: ItemPathIdInterface,
    pedestal: StandardPedestal,
) -> StandardTrackedExcepted<T>
where
    T: FromValue,
{
    dev_eval_context()
        .eval_lazy_val(item_path_id_interface, pedestal)
        .map(T::from_value_static)
}

pub fn eval_generic_gn_with<T>(
    ki_repr_interface: KiReprInterface,
    pedestal: StandardPedestal,
    f: impl FnOnce() -> StandardLinketImplKiControlFlow,
) -> StandardTrackedExcepted<T>
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
    f: fn(&'static __Self) -> StandardLinketImplKiControlFlow,
) -> StandardTrackedExcepted<T>
where
    T: FromValue,
{
    dev_eval_context()
        .eval_memo_field_with(item_path_id_interface, __self, f)
        .map(T::from_value_static)
}
