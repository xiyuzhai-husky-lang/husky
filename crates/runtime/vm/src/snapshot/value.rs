use crate::*;

#[derive(Clone)]
pub enum StackValueSnapshot<'eval> {
    Copyable(CopyableValue),
    GlobalPure(Arc<dyn AnyValueDyn<'eval>>),
    Owned(OwnedValue<'eval>),
    Shared(Arc<dyn AnyValueDyn<'eval>>),
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
            StackValueSnapshot::Shared(value) => &**value,
            StackValueSnapshot::MutRef { .. } => todo!(),
            StackValueSnapshot::Uninitialized => todo!(),
        }
    }

    pub fn eval(&self) -> EvalValue<'eval> {
        todo!()
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
            StackValueSnapshot::Shared(value) => todo!(),
            StackValueSnapshot::Uninitialized => todo!(),
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
            StackValueSnapshot::Shared(value) => todo!(),
            StackValueSnapshot::Uninitialized => todo!(),
        }
    }
}
