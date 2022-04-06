use crate::*;

#[derive(Debug, Clone)]
pub enum StructMembValue<'eval> {
    Primitive(PrimitiveValue),
    Boxed(BoxedValue<'eval>),
    GlobalPure(Arc<dyn AnyValueDyn<'eval>>),
    GlobalRef(&'eval dyn AnyValueDyn<'eval>),
    Moved,
}

impl<'eval> PartialEq for StructMembValue<'eval> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Primitive(l0), Self::Primitive(r0)) => l0 == r0,
            (Self::Boxed(l0), Self::Boxed(r0)) => l0 == r0,
            (Self::GlobalPure(l0), Self::GlobalPure(r0)) => todo!(),
            (Self::GlobalRef(l0), Self::GlobalRef(r0)) => todo!(),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl<'eval> Eq for StructMembValue<'eval> {}

impl<'stack, 'eval: 'stack> StructMembValue<'eval> {
    pub fn into_stack(self) -> StackValue<'stack, 'eval> {
        todo!()
    }

    pub fn share_globally(&self) -> EvalValue<'eval> {
        todo!()
    }
}
