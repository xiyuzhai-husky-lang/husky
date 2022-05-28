use crate::*;

use super::*;

// inner ops
impl<'a, 'b> AtomParser<'a, 'b> {
    pub(crate) fn push(&mut self, kind: AtomVariant) -> AtomResult<()> {
        self.stack
            .push(Atom::new(self.token_stream.pop_text_range(), kind))
    }

    pub(crate) fn save_stream(&self) -> TokenStream<'b> {
        self.token_stream.clone()
    }

    pub(crate) fn rollback(&mut self, stream: TokenStream<'b>) {
        *self.token_stream = stream
    }
}
