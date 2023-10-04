use super::*;
use std::num::NonZeroU8;

#[derive(Debug, Clone, Copy)]
pub struct DocId(NonZeroU8);

impl DocId {
    pub fn index(self) -> u8 {
        self.0.get() - 1
    }
}

#[derive(Default)]
pub(crate) struct Docs {
    docs: Vec<Doc>,
}

impl std::ops::Index<DocId> for Docs {
    type Output = Doc;

    fn index(&self, index: DocId) -> &Self::Output {
        todo!()
    }
}

impl std::ops::IndexMut<DocId> for Docs {
    fn index_mut(&mut self, index: DocId) -> &mut Self::Output {
        todo!()
    }
}
