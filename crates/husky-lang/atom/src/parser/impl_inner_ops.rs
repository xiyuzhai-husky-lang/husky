use crate::*;

use super::*;

// inner ops
impl<'a> ScopeLRParser<'a> {
    pub(super) fn push(&mut self, kind: AtomKind) -> AtomResult<()> {
        self.atom_group
            .push(Atom::new(self.stream.pop_range(), kind))
    }

    pub(super) fn save_stream(&self) -> Stream<'a> {
        self.stream.clone()
    }

    pub(super) fn rollback(&mut self, stream: Stream<'a>) {
        self.stream = stream
    }
}
