use crate::*;

#[derive(Clone)]
pub enum StackValueSnapshot<'eval> {
    Primitive(PrimitiveValue),
    GlobalPure(Arc<dyn AnyValueDyn<'eval>>),
    Boxed(BoxedValue<'eval>),
    MutRef {
        value: Arc<dyn AnyValueDyn<'eval>>,
        owner: StackIdx,
        gen: MutRefGenerator,
    },
}

impl<'eval> StackValueSnapshot<'eval> {
    pub fn any_ref(&self) -> &dyn AnyValueDyn<'eval> {
        match self {
            StackValueSnapshot::Primitive(_) => todo!(),
            StackValueSnapshot::GlobalPure(_) => todo!(),
            StackValueSnapshot::Boxed(boxed_value) => boxed_value.any_ref(),
            StackValueSnapshot::MutRef { value, owner, gen } => todo!(),
        }
    }
}

impl<'eval> std::fmt::Debug for StackValueSnapshot<'eval> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackValueSnapshot::Primitive(arg0) => f
                .debug_tuple("StackValueSnapshot::Primitive")
                .field(arg0)
                .finish(),
            StackValueSnapshot::MutRef { value, owner, .. } => f
                .debug_struct("StackValueSnapshot::MutRef")
                .field("value", value)
                .field("owner", owner)
                .finish(),
            StackValueSnapshot::GlobalPure(value) => f
                .debug_struct("StackValueSnapshot::GlobalPure")
                .field("value", value)
                .finish(),
            StackValueSnapshot::Boxed(value) => f
                .debug_struct("StackValueSnapshot::Boxed")
                .field("value", value)
                .finish(),
        }
    }
}

impl<'eval> From<PrimitiveValue> for StackValueSnapshot<'eval> {
    fn from(value: PrimitiveValue) -> Self {
        Self::Primitive(value)
    }
}

impl<'stack, 'eval: 'stack> Into<StackValue<'stack, 'eval>> for &StackValueSnapshot<'eval> {
    fn into(self) -> StackValue<'stack, 'eval> {
        match self {
            StackValueSnapshot::Primitive(value) => StackValue::Primitive(*value),
            StackValueSnapshot::MutRef { owner, gen, .. } => todo!(),
            StackValueSnapshot::GlobalPure(value) => StackValue::GlobalPure(value.clone()),
            StackValueSnapshot::Boxed(value) => StackValue::Boxed(value.clone()),
        }
    }
}
