use crate::*;

#[derive(Clone)]
pub enum StackValueSnapshot<'eval> {
    Copyable(CopyableValue),
    GlobalPure(Arc<dyn AnyValueDyn<'eval>>),
    GlobalRef(&'eval dyn AnyValueDyn<'eval>),
    Owned(OwnedValue<'eval>),
    MutRef {
        owner: StackIdx,
        gen: MutRefGenerator,
    },
    Uninitialized,
}

impl<'eval> StackValueSnapshot<'eval> {
    pub fn any_ref(&self) -> &dyn AnyValueDyn<'eval> {
        match self {
            StackValueSnapshot::Copyable(value) => value.any_ref(),
            StackValueSnapshot::GlobalPure(value) => &**value,
            StackValueSnapshot::Owned(boxed_value) => boxed_value.any_ref(),
            StackValueSnapshot::Owned(snapshared_value) => snapshared_value.any_ref(),
            StackValueSnapshot::MutRef { .. } => todo!(),
            StackValueSnapshot::Uninitialized => todo!(),
            StackValueSnapshot::GlobalRef(_) => todo!(),
        }
    }

    pub fn eval(&self) -> EvalValue<'eval> {
        match self {
            StackValueSnapshot::Copyable(copyable_value) => EvalValue::Copyable(*copyable_value),
            StackValueSnapshot::GlobalPure(value) => EvalValue::GlobalPure(value.clone()),
            StackValueSnapshot::GlobalRef(value) => EvalValue::GlobalRef(*value),
            StackValueSnapshot::Owned(value) => EvalValue::Owned(value.clone()),
            StackValueSnapshot::MutRef { owner, gen } => todo!(),
            StackValueSnapshot::Uninitialized => todo!(),
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
            StackValueSnapshot::MutRef { owner, .. } => todo!(),
            StackValueSnapshot::GlobalPure(value) => f
                .debug_struct("StackValueSnapshot::GlobalPure")
                .field("value", value)
                .finish(),
            StackValueSnapshot::Owned(value) => f
                .debug_struct("StackValueSnapshot::Boxed")
                .field("value", value)
                .finish(),
            StackValueSnapshot::Owned(value) => todo!(),
            StackValueSnapshot::Uninitialized => todo!(),
            StackValueSnapshot::GlobalRef(_) => todo!(),
        }
    }
}

impl<'eval> From<CopyableValue> for StackValueSnapshot<'eval> {
    fn from(value: CopyableValue) -> Self {
        Self::Copyable(value)
    }
}

impl<'stack, 'eval: 'stack> Into<StackValue<'stack, 'eval>> for &StackValueSnapshot<'eval> {
    fn into(self) -> StackValue<'stack, 'eval> {
        match self {
            StackValueSnapshot::Copyable(value) => StackValue::Copyable(*value),
            StackValueSnapshot::MutRef { owner, gen, .. } => todo!(),
            StackValueSnapshot::GlobalPure(value) => StackValue::GlobalPure(value.clone()),
            StackValueSnapshot::Owned(value) => StackValue::Owned(value.clone()),
            StackValueSnapshot::Owned(value) => todo!(),
            StackValueSnapshot::Uninitialized => todo!(),
            StackValueSnapshot::GlobalRef(_) => todo!(),
        }
    }
}
