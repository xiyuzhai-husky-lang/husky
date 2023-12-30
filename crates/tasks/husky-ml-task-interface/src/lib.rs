pub mod label;
pub mod ugly;

use husky_linkage_impl::standard::ValControlFlow;
pub use husky_ml_task_interface_macros::*;

use husky_standard_value::{ugly::__ValueStands, FromValue, Value};
use husky_task_interface::val_repr::{
    ValDomainReprInterface, ValReprInterface, ValRuntimeConstantInterface,
};
use husky_trace_protocol::protocol::IsPedestal;
use serde::{Deserialize, Serialize};
use shifted_unsigned_int::ShiftedU32;
use std::{cell::Cell, convert::Infallible, thread::LocalKey};

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

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MlPedestal {
    Specific(InputId),
    Generic,
}

impl Default for MlPedestal {
    fn default() -> Self {
        MlPedestal::Specific(InputId::from_index(0))
    }
}

impl IsPedestal for MlPedestal {}

impl MlPedestal {
    pub fn input_id(self) -> Option<InputId> {
        match self {
            MlPedestal::Specific(input_id) => Some(input_id),
            MlPedestal::Generic => None,
        }
    }
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

pub fn eval_val_repr<T>(
    val_repr: ValReprInterface,
    value_stands: Option<&mut __ValueStands>,
) -> ValControlFlow<T>
where
    T: FromValue + 'static,
{
    let mut value = dev_eval_context().eval_val_repr(val_repr)?;
    ValControlFlow::Continue(<T as FromValue>::from_value_aux(value, value_stands))
}

pub fn eval_val_repr_at_input<T>(
    val_repr: ValReprInterface,
    input_id: InputId,
    value_stands: Option<&mut __ValueStands>,
) -> ValControlFlow<T>
where
    T: FromValue + 'static,
{
    with_dev_eval_context(dev_eval_context().with_pedestal(input_id.into()), || {
        eval_val_repr(val_repr, value_stands)
    })
}

pub fn eval_val_domain_repr_at_input(
    val_domain_repr: ValDomainReprInterface,
    input_id: InputId,
    value_stands: Option<&mut __ValueStands>,
) -> ValControlFlow<(), Infallible> {
    match val_domain_repr {
        ValDomainReprInterface::Omni => ValControlFlow::Continue(()),
        ValDomainReprInterface::ConditionSatisfied(_) => todo!(),
        ValDomainReprInterface::ConditionNotSatisfied(_) => todo!(),
        ValDomainReprInterface::StmtNotReturned(stmt_val_repr) => {
            match eval_val_repr_at_input::<()>(stmt_val_repr, input_id, value_stands) {
                ValControlFlow::Continue(_) => ValControlFlow::Continue(()),
                ValControlFlow::LoopContinue => todo!(),
                ValControlFlow::LoopExit(_) => todo!(),
                ValControlFlow::Return(_) | ValControlFlow::Undefined => ValControlFlow::Undefined,
                ValControlFlow::Err(_) => todo!(),
            }
        }
    }
}

pub fn eval_val_runtime_constant<T>(val_runtime_constant: ValRuntimeConstantInterface) -> T
where
    T: FromValue,
{
    // no need to return a stand, because runtime constant are always solid
    T::from_value_static(dev_eval_context().eval_val_runtime_constant(val_runtime_constant))
}
