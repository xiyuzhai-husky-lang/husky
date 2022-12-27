use super::*;

impl<'a, 'b, 'c> ExprParser<'a, 'b, 'c> {
    pub(super) fn try_get_identifier(&mut self) -> Option<(TokenIdx, Identifier)> {
        self.token_iter
            .try_get_one_token_with_indexed(|token_kind| match token_kind {
                TokenKind::Identifier(ident) => Some(*ident),
                _ => None,
            })
    }
}
