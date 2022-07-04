use crate::*;

#[derive(Clone)]
pub enum StackValueSnapshot<'eval> {
    Copyable(CopyableValue),
    EvalPure(Arc<dyn AnyValueDyn<'eval> + 'eval>),
    EvalRef(__EvalRef<'eval>),
    Owned(__OwnedValue<'eval, 'eval>),
    FullyOwnedRef(Arc<dyn AnyValueDyn<'eval> + 'eval>),
    RefMut {
        value: EvalValue<'eval>,
        owner: VMStackIdx,
        gen: MutRefGenerator,
    },
}

impl<'eval> StackValueSnapshot<'eval> {
    pub fn any_ref(&self) -> &(dyn AnyValueDyn<'eval> + 'eval) {
        match self {
            StackValueSnapshot::Copyable(value) => value.any_ref(),
            StackValueSnapshot::EvalPure(value) => &**value,
            StackValueSnapshot::Owned(boxed_value) => boxed_value.any_ref(),
            StackValueSnapshot::Owned(snapshared_value) => snapshared_value.any_ref(),
            StackValueSnapshot::RefMut { value, .. } => value.any_ref(),
            StackValueSnapshot::EvalRef(value) => value.0,
            StackValueSnapshot::FullyOwnedRef(_) => todo!(),
        }
    }

    pub fn eval(&self) -> EvalValue<'eval> {
        match self {
            StackValueSnapshot::Copyable(copyable_value) => EvalValue::Copyable(*copyable_value),
            StackValueSnapshot::EvalPure(value) => EvalValue::EvalPure(value.clone()),
            StackValueSnapshot::EvalRef(value) => EvalValue::EvalRef(*value),
            StackValueSnapshot::Owned(value) => EvalValue::Owned(value.clone()),
            StackValueSnapshot::RefMut { value, owner, gen } => value.clone(),
            StackValueSnapshot::FullyOwnedRef(_) => todo!(),
        }
    }
}

impl<'eval> std::fmt::Debug for StackValueSnapshot<'eval> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackValueSnapshot::Copyable(arg0) => f
                .debug_tuple("StackValueSnapshot::Primitive")
                .field(arg0)
                .finish(),
            StackValueSnapshot::RefMut { value, owner, .. } => f
                .debug_struct("StackValueSnapshot::RefMut")
                .field("value", value)
                .finish(),
            StackValueSnapshot::EvalPure(value) => f
                .debug_struct("StackValueSnapshot::GlobalPure")
                .field("value", value)
                .finish(),
            StackValueSnapshot::Owned(value) => f
                .debug_struct("StackValueSnapshot::Owned")
                .field("value", value)
                .finish(),
            StackValueSnapshot::EvalRef(value) => f
                .debug_struct("StackValueSnapshot::EvalRef")
                .field("value", value)
                .finish(),
            StackValueSnapshot::FullyOwnedRef(_) => todo!(),
        }
    }
}

impl<'eval> From<CopyableValue> for StackValueSnapshot<'eval> {
    fn from(value: CopyableValue) -> Self {
        Self::Copyable(value)
    }
}

impl<'temp, 'eval: 'temp> Into<__TempValue<'temp, 'eval>> for &StackValueSnapshot<'eval> {
    fn into(self) -> __TempValue<'temp, 'eval> {
        match self {
            StackValueSnapshot::Copyable(value) => __TempValue::Copyable(*value),
            StackValueSnapshot::RefMut { owner, gen, .. } => todo!(),
            StackValueSnapshot::EvalPure(value) => __TempValue::EvalPure(value.clone()),
            StackValueSnapshot::Owned(value) => __TempValue::OwnedEval(value.clone()),
            StackValueSnapshot::EvalRef(value) => __TempValue::EvalRef(*value),
            StackValueSnapshot::FullyOwnedRef(value) => {
                __TempValue::OwnedEval(value.clone_into_box_dyn().into())
            }
        }
    }
}
