use super::*;
use husky_codespan_utils::emit_to_stdout;
use latex_token::idx::LxTokenIdx;

impl<'db> VdSynExprBuilder<'db> {
    pub(crate) fn emit_message_over_token_to_stdout(&self, token_idx: LxTokenIdx, message: String) {
        emit_to_stdout(
            self.content,
            self.token_storage[token_idx]
                .text_offset_range()
                .raw_range(),
            message,
        );
    }
}
