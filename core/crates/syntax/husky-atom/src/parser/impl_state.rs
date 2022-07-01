use husky_text::TextPosition;
use husky_token::TokenStreamState;

use crate::*;

use super::*;

// inner ops
impl<'a, 'b> AtomParser<'a, 'b> {
    pub(crate) fn push(&mut self, kind: AtomVariant, text_start: TextPosition) -> AtomResult<()> {
        self.stack.push(HuskyAtom::new(
            self.token_stream.text_range(text_start),
            kind,
        ))
    }

    pub fn save_state(&self) -> AtomParserState {
        AtomParserState {
            token_stream_state: self.token_stream.save_state(),
            atom_context_state: self.atom_context.save_state(),
        }
    }

    pub fn rollback(&mut self, state: AtomParserState) {
        self.token_stream.rollback(state.token_stream_state);
        self.atom_context.rollback(state.atom_context_state)
    }
}

pub struct AtomParserState {
    token_stream_state: TokenStreamState,
    atom_context_state: AtomContextState,
}
