use crate::*;

#[derive(Debug, Clone)]
pub struct StackSnapshot {
    pub message: String, // for debug
    pub(crate) values: Vec<__RegularValue>,
}

impl StackSnapshot {
    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl From<&StackSnapshot> for VMStack {
    fn from(val: &StackSnapshot) -> Self {
        VMStack::new(
            val.values
                .iter()
                .map(|value_snapshot| value_snapshot.clone()),
        )
    }
}

impl std::ops::Index<VMStackIdx> for StackSnapshot {
    type Output = __RegularValue;

    fn index(&self, index: VMStackIdx) -> &Self::Output {
        &self.values[index.raw()]
    }
}
