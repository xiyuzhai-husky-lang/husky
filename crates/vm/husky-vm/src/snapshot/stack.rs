use crate::*;

#[derive(Debug, Clone)]
pub struct StackSnapshot<'eval> {
    pub message: String, // for debug
    pub(crate) values: Vec<__Register<'eval>>,
}

impl<'eval> StackSnapshot<'eval> {
    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl<'eval> From<&StackSnapshot<'eval>> for VMStack<'eval> {
    fn from(val: &StackSnapshot<'eval>) -> Self {
        VMStack::new(
            val.values
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
