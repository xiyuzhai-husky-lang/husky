use crate::*;

#[derive(Clone)]
pub enum StackValueSnapshot {
    Primitive(PrimitiveValue),
    MutRef {
        value: Arc<dyn AnyValueDyn>,
        owner: StackIdx,
        gen: Arc<dyn (Fn(&mut StackValue) -> *mut dyn AnyValueDyn) + Send + Sync + 'static>,
    },
}

impl std::fmt::Debug for StackValueSnapshot {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Primitive(arg0) => f.debug_tuple("Primitive").field(arg0).finish(),
            Self::MutRef { value, owner, .. } => {
                f.debug_struct("MutRef").field("owner", owner).finish()
            }
        }
    }
}

impl From<PrimitiveValue> for StackValueSnapshot {
    fn from(value: PrimitiveValue) -> Self {
        Self::Primitive(value)
    }
}

impl<'stack, 'eval: 'stack> Into<StackValue<'stack, 'eval>> for &StackValueSnapshot {
    fn into(self) -> StackValue<'stack, 'eval> {
        match self {
            StackValueSnapshot::Primitive(value) => StackValue::Primitive(*value),
            StackValueSnapshot::MutRef { owner, gen, .. } => todo!(),
        }
    }
}
