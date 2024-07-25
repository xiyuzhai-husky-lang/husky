use crate::{pedestal::StandardPedestal, DevEvalContext};
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
static mut EVAL_CONTEXT: Option<DevEvalContext> = None;

impl IsDevsoulInterface for StandardDevsoulInterface {
    type LinketImpl = StandardLinketImpl<StandardPedestal>;

    fn set_dev_eval_context(ctx: DevEvalContext) {
        unsafe {
            assert!(EVAL_CONTEXT.is_none());
            EVAL_CONTEXT = Some(ctx);
        }
    }

    fn eval_context() -> DevEvalContext {
        unsafe { EVAL_CONTEXT.expect("`EVAL_CONTEXT` not initialized!!!") }
    }
}

fn eval_context() -> DevEvalContext {
    unsafe { EVAL_CONTEXT.expect("`EVAL_CONTEXT` not initialized!!!") }
}

pub fn eval_eager_val_with<T>(
    item_path_id_interface: ItemPathIdInterface,
    pedestal: StandardPedestal,
    f: fn() -> StandardLinketImplKiControlFlow,
) -> T
where
    T: FromValue,
{
    T::from_value_static(eval_context().eval_eager_val_with(item_path_id_interface, pedestal, f))
}

pub fn eval_lazy_val<T>(
    item_path_id_interface: ItemPathIdInterface,
    pedestal: StandardPedestal,
) -> T
where
    T: FromValue,
{
    T::from_value_static(eval_context().eval_lazy_val(item_path_id_interface, pedestal))
}
