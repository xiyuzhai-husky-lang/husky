use arrayvec::ArrayVec;

use crate::{stack::STACK_SIZE, *};

#[derive(Debug, Clone)]
pub struct StackSnapshot {
    pub(crate) values: Vec<StackValueSnapshot>,
}

impl StackSnapshot {
    pub(crate) fn stack<'stack, 'eval: 'stack>(&self) -> VMStack<'stack, 'eval> {
        todo!()
    }
}

impl<'stack, 'eval: 'stack> Into<VMStack<'stack, 'eval>> for &StackSnapshot {
    fn into(self) -> VMStack<'stack, 'eval> {
        VMStack::new(
            self.values
                .iter()
                .map(|value_snapshot| value_snapshot.into()),
        )
    }
}
