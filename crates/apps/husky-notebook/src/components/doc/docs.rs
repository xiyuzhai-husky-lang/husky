use super::*;
use std::num::NonZeroU8;

#[derive(Debug, Clone, Copy)]
pub struct DocId(NonZeroU8);

impl DocId {
    fn new(index: usize) -> Self {
        debug_assert!(index < (i8::MAX as usize) - 1);
        Self(unsafe { NonZeroU8::new_unchecked((index as u8) + 1) })
    }

    pub fn index(self) -> usize {
        (self.0.get() - 1) as usize
    }
}

#[derive(Default)]
pub(crate) struct Docs {
    arena: Vec<Doc>,
}

impl std::ops::Index<DocId> for Docs {
    type Output = Doc;

    fn index(&self, id: DocId) -> &Self::Output {
        &self.arena[id.index()]
    }
}

impl std::ops::IndexMut<DocId> for Docs {
    fn index_mut(&mut self, id: DocId) -> &mut Self::Output {
        &mut self.arena[id.index()]
    }
}

impl Docs {
    pub(super) fn alloc(&mut self, doc: Doc) -> DocId {
        let index = self.arena.len();
        self.arena.push(doc);
        DocId::new(index)
    }
}
