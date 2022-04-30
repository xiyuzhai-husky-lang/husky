use crate::*;

use super::*;

// inner ops
impl<'a> AtomLRParser<'a> {
    pub(crate) fn push(&mut self, kind: AtomVariant) -> AtomResult<()> {
        self.stack.push(Atom::new(self.stream.pop_range(), kind))
    }

    pub(crate) fn save_stream(&self) -> TokenStream<'a> {
        self.stream.clone()
    }

    pub(crate) fn rollback(&mut self, stream: TokenStream<'a>) {
        self.stream = stream
    }
}
