use super::*;
use husky_codespan_utils::emit_to_stdout;
use latex_token::idx::LxTokenIdx;

impl<'a> VdSemExprBuilder<'a> {
    pub(crate) fn emit_message_over_token_to_stdout(&self, token_idx: LxTokenIdx, message: String) {
        emit_to_stdout(
            self.content,
            self.token_storage[token_idx]
                .text_offset_range()
                .raw_range(),
            message,
        );
    }
    pub(crate) fn emit_message_over_expr_to_stdout(&self, expr: VdSynExprIdx, message: String) {
        let range = match self.syn_expr_range_map[expr] {
            VdSynExprTokenIdxRange::Standard(lx_token_idx_range) => lx_token_idx_range,
        };
        emit_to_stdout(
            self.content,
            self.token_storage
                .token_idx_range_offset_range(range)
                .raw_range(),
            message,
        );
    }
}
