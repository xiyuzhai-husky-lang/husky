pub mod label;
pub mod pedestal;
pub mod ugly;

use self::pedestal::MlPedestal;
use husky_linkage_impl::standard::StandardLinkageImplValControlFlow;
use husky_standard_value::{ugly::__ValueStands, FromValue, Value};
use husky_task_interface::{
    pedestal::IsPedestal,
    val_repr::{ValDomainReprInterface, ValReprInterface, ValRuntimeConstantInterface},
};
use serde::{Deserialize, Serialize};
use shifted_unsigned_int::ShiftedU32;
use std::{cell::Cell, convert::Infallible};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub struct InputId(ShiftedU32);

impl InputId {
    pub fn from_index(index: usize) -> Self {
        Self(index.into())
    }

    pub fn index(self) -> usize {
        self.0.into()
    }
}

/// panics if dev eval context is empty
#[track_caller]
pub fn input_id() -> InputId {
    DEV_EVAL_CONTEXT
        .get()
        .unwrap()
        .pedestal()
        .input_id()
        .expect("pedestal is generic, no input id")
}

#[test]
fn sample_id_size_works() {
    assert_eq!(std::mem::size_of::<InputId>(), std::mem::size_of::<u32>());
    assert_eq!(
        std::mem::size_of::<Option<InputId>>(),
        std::mem::size_of::<u32>()
    )
}

pub type DevEvalContext =
    husky_task_interface::DevEvalContext<husky_linkage_impl::standard::LinkageImpl<MlPedestal>>;

thread_local! {
    pub static DEV_EVAL_CONTEXT: Cell<std::option::Option<DevEvalContext>> = Cell::new(None);
}

pub fn dev_eval_context() -> DevEvalContext {
    DEV_EVAL_CONTEXT.get().expect("`DEV_EVAL_CONTEXT` not set")
}

pub fn with_dev_eval_context<R>(ctx: DevEvalContext, f: impl FnOnce() -> R) -> R {
    let old = DEV_EVAL_CONTEXT.replace(Some(ctx));
    let r = f();
    DEV_EVAL_CONTEXT.set(old);
    r
}

pub fn eval_val_repr_interface<T>(
    val_repr: ValReprInterface,
    value_stands: Option<&mut __ValueStands>,
) -> StandardLinkageImplValControlFlow<T>
where
    T: FromValue + 'static,
{
    let value = dev_eval_context().eval_val_repr_interface(val_repr)?;
    StandardLinkageImplValControlFlow::Continue(<T as FromValue>::from_value_aux(
        value,
        value_stands,
    ))
}

pub fn eval_val_domain_repr_interface(
    val_domain_repr_interface: ValDomainReprInterface,
) -> StandardLinkageImplValControlFlow<(), Infallible> {
    dev_eval_context().eval_val_domain_repr_interface(val_domain_repr_interface)
}

pub fn eval_val_repr_interface_at_input<T>(
    val_repr_interface: ValReprInterface,
    input_id: InputId,
    value_stands: Option<&mut __ValueStands>,
) -> StandardLinkageImplValControlFlow<T>
where
    T: FromValue + 'static,
{
    with_dev_eval_context(dev_eval_context().with_pedestal(input_id.into()), || {
        eval_val_repr_interface(val_repr_interface, value_stands)
    })
}

pub fn eval_val_domain_repr_interface_at_input(
    val_domain_repr_interface: ValDomainReprInterface,
    input_id: InputId,
) -> StandardLinkageImplValControlFlow<(), Infallible> {
    with_dev_eval_context(dev_eval_context().with_pedestal(input_id.into()), || {
        eval_val_domain_repr_interface(val_domain_repr_interface)
    })
}

pub fn eval_val_runtime_constant<T>(val_runtime_constant: ValRuntimeConstantInterface) -> T
where
    T: FromValue,
{
    // no need to return a stand, because runtime constant are always solid
    T::from_value_static(dev_eval_context().eval_val_runtime_constant(val_runtime_constant))
}
