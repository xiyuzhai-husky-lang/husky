use crate::*;

#[derive(Clone)]
pub enum StackValueSnapshot<'eval> {
    Copyable(CopyableValue),
    GlobalPure(Arc<dyn AnyValueDyn<'eval> + 'eval>),
    GlobalRef(&'eval (dyn AnyValueDyn<'eval> + 'eval)),
    Owned(OwnedValue<'eval, 'eval>),
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
            StackValueSnapshot::GlobalPure(value) => &**value,
            StackValueSnapshot::Owned(boxed_value) => boxed_value.any_ref(),
            StackValueSnapshot::Owned(snapshared_value) => snapshared_value.any_ref(),
            StackValueSnapshot::RefMut { value, .. } => value.any_ref(),
            StackValueSnapshot::GlobalRef(value) => *value,
        }
    }

    pub fn eval(&self) -> EvalValue<'eval> {
        match self {
            StackValueSnapshot::Copyable(copyable_value) => EvalValue::Copyable(*copyable_value),
            StackValueSnapshot::GlobalPure(value) => EvalValue::GlobalPure(value.clone()),
            StackValueSnapshot::GlobalRef(value) => EvalValue::GlobalRef(*value),
            StackValueSnapshot::Owned(value) => EvalValue::Owned(value.clone()),
            StackValueSnapshot::RefMut { value, owner, gen } => value.clone(),
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
            StackValueSnapshot::GlobalPure(value) => f
                .debug_struct("StackValueSnapshot::GlobalPure")
                .field("value", value)
                .finish(),
            StackValueSnapshot::Owned(value) => f
                .debug_struct("StackValueSnapshot::Owned")
                .field("value", value)
                .finish(),
            StackValueSnapshot::GlobalRef(value) => f
                .debug_struct("StackValueSnapshot::GlobalRef")
                .field("value", value)
                .finish(),
        }
    }
}

impl<'eval> From<CopyableValue> for StackValueSnapshot<'eval> {
    fn from(value: CopyableValue) -> Self {
        Self::Copyable(value)
    }
}

impl<'vm, 'eval: 'vm> Into<VMValue<'vm, 'eval>> for &StackValueSnapshot<'eval> {
    fn into(self) -> VMValue<'vm, 'eval> {
        match self {
            StackValueSnapshot::Copyable(value) => VMValue::Copyable(*value),
            StackValueSnapshot::RefMut { owner, gen, .. } => todo!(),
            StackValueSnapshot::GlobalPure(value) => VMValue::EvalPure(value.clone()),
            StackValueSnapshot::Owned(value) => VMValue::FullyOwned(value.clone()),
            StackValueSnapshot::GlobalRef(value) => VMValue::GlobalRef(*value),
        }
    }
}
