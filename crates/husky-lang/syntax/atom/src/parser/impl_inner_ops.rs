use crate::*;

use super::*;

// inner ops
impl<'a> ScopeLRParser<'a> {
    pub(crate) fn push(&mut self, kind: AtomKind) -> AtomResult<()> {
        self.stack.push(Atom::new(self.stream.pop_range(), kind))
    }

    pub(crate) fn save_stream(&self) -> Stream<'a> {
        self.stream.clone()
    }

    pub(crate) fn rollback(&mut self, stream: Stream<'a>) {
        self.stream = stream
    }
}
