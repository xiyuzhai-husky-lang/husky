use super::*;

impl<'a> AstParser<'a> {
    pub(super) fn parse_uses(&mut self, token_group_idx: TokenGroupIdx, indent: u32) -> Ast {
        Ast::Use { token_group_idx }
    }
}
