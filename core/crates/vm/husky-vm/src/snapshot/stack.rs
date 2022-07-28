use crate::*;

#[derive(Debug, Clone)]
pub struct StackSnapshot<'eval> {
    pub message: String, // for debug
    pub(crate) values: Vec<__Register<'eval>>,
}

impl<'eval> StackSnapshot<'eval> {
    pub(crate) fn stack(&self) -> VMStack {
        todo!()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl<'eval> Into<VMStack<'eval>> for &StackSnapshot<'eval> {
    fn into(self) -> VMStack<'eval> {
        VMStack::new(
            self.values
                .iter()
                .map(|value_snapshot| value_snapshot.clone()),
        )
    }
}

impl<'eval> std::ops::Index<VMStackIdx> for StackSnapshot<'eval> {
    type Output = __Register<'eval>;

    fn index(&self, index: VMStackIdx) -> &Self::Output {
        &self.values[index.raw()]
    }
}
