use crate::*;
use husky_token_syntax::AbsSemanticToken;

impl<'a> AstTransformer<'a> {
    pub(super) fn push_abs_semantic_token(&mut self, new_token: AbsSemanticToken) {
        if self.abs_semantic_tokens.len() > 0 {
            let last_end = self.abs_semantic_tokens.last().unwrap().range.end;
            let new_start = new_token.range.start;
            should!(last_end.i() <= new_start.i());
            if last_end.i() == new_start.i() {
                should!(last_end.j() <= new_start.j());
            }
        }
        self.abs_semantic_tokens.push(new_token)
    }

    pub(super) fn insert_abs_semantic_token(&mut self, new_token: AbsSemanticToken) {
        let index = self
            .abs_semantic_tokens
            .iter()
            .position(|token| token.range.start > new_token.range.start)
            .unwrap_or(self.abs_semantic_tokens.len());
        self.abs_semantic_tokens.insert(index, new_token)
    }
}
