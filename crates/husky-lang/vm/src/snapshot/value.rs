use crate::*;

#[derive(Clone)]
pub enum StackValueSnapshot<'eval> {
    Primitive(PrimitiveValue),
    MutRef {
        value: Arc<dyn AnyValueDyn<'eval>>,
        owner: StackIdx,
        gen: MutRefGenerator,
    },
}

impl<'eval> std::fmt::Debug for StackValueSnapshot<'eval> {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Primitive(arg0) => f.debug_tuple("Primitive").field(arg0).finish(),
            Self::MutRef { value, owner, .. } => {
                f.debug_struct("MutRef").field("owner", owner).finish()
            }
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
        }
    }
}
