use crate::*;

#[derive(Debug, Clone)]
pub struct StackSnapshot<'eval> {
    pub message: String, // for debug
    pub(crate) values: Vec<StackValueSnapshot<'eval>>,
}

impl<'vm, 'eval: 'vm> StackSnapshot<'eval> {
    pub(crate) fn stack(&self) -> VMStack<'vm, 'eval> {
        todo!()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl<'vm, 'eval: 'vm> Into<VMStack<'vm, 'eval>> for &StackSnapshot<'eval> {
    fn into(self) -> VMStack<'vm, 'eval> {
        VMStack::new(
            self.values
                .iter()
                .map(|value_snapshot| value_snapshot.into()),
        )
    }
}

impl<'eval> std::ops::Index<VMStackIdx> for StackSnapshot<'eval> {
    type Output = StackValueSnapshot<'eval>;

    fn index(&self, index: VMStackIdx) -> &Self::Output {
        &self.values[index.raw()]
    }
}
