use crate::*;

#[derive(Debug, Clone)]
pub struct StackSnapshot<'eval> {
    pub(crate) values: Vec<StackValueSnapshot<'eval>>,
}

impl<'stack, 'eval: 'stack> StackSnapshot<'eval> {
    pub(crate) fn stack(&self) -> VMStack<'stack, 'eval> {
        todo!()
    }
}

impl<'stack, 'eval: 'stack> Into<VMStack<'stack, 'eval>> for &StackSnapshot<'eval> {
    fn into(self) -> VMStack<'stack, 'eval> {
        VMStack::new(
            self.values
                .iter()
                .map(|value_snapshot| value_snapshot.into()),
        )
    }
}
