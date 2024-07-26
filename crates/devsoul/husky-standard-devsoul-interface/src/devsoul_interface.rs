use crate::{dev_eval_context, pedestal::StandardPedestal, set_dev_eval_context, DevEvalContext};
use husky_devsoul_interface::ki_repr::KiDomainReprInterface;
use husky_devsoul_interface::ki_repr::KiReprInterface;
use husky_devsoul_interface::{
    devsoul::IsDevsoulInterface, item_path::ItemPathIdInterface, KiControlFlow,
};
use husky_linket_impl::standard::{StandardLinketImpl, StandardLinketImplKiControlFlow};
use husky_standard_value::FromValue;
use std::cell::OnceCell;

pub struct StandardDevsoulInterface {}

/// It looks like this definition is not safe.
///
/// However, the program is only going to touch this place in a mutable way, and in a sequential manner.
///
/// Then it will become immutable effectively;

impl IsDevsoulInterface for StandardDevsoulInterface {
    type LinketImpl = StandardLinketImpl<StandardPedestal>;

    fn set_dev_eval_context(ctx: DevEvalContext) {
        set_dev_eval_context(ctx)
    }

    fn dev_eval_context() -> DevEvalContext {
        dev_eval_context()
    }
}

pub fn eval_eager_val_with<T>(
    item_path_id_interface: ItemPathIdInterface,
    pedestal: StandardPedestal,
    f: fn() -> StandardLinketImplKiControlFlow,
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
    f: impl FnOnce() -> StandardLinketImplKiControlFlow,
) -> T
where
    T: FromValue,
{
    T::from_value_static(dev_eval_context().eval_generic_gn_with(ki_repr_interface, pedestal, f))
}
