use super::*;
use husky_entity_card::TyKingdom;
use husky_opn_syntax::Bracket;
use husky_token::{Decorator, Token, TypeKeyword};
use std::iter::Peekable;

impl<'a> AstParser<'a> {
    pub(super) fn parse_defn(&mut self, token_group_idx: TokenGroupIdx, indent: u32) -> Ast {
        self.parse_defn_aux(indent, token_group_idx)
            .unwrap_or_else(|error| Ast::Err {
                token_group_idx,
                error,
            })
    }

    fn parse_defn_aux(&mut self, indent: u32, token_group_idx: TokenGroupIdx) -> AstResult<Ast> {
        let (body, body_kind) = {
            let body = self.parse_asts(indent + INDENT_INCR);
            match body.last() {
                Some(_) => (body, DefnBodyKind::Block),
                None => match self.token_groups.peek_with_exact_indent(indent) {
                    Some((token_group_idx, token_group)) => match token_group.first().kind {
                        TokenKind::Special(SpecialToken::Vertical) => {
                            (self.parse_case_stmts(indent), DefnBodyKind::Cases)
                        }
                        _ => (Default::default(), DefnBodyKind::None),
                    },
                    None => (Default::default(), DefnBodyKind::None),
                },
            }
        };
        let mut aux_parser = self.aux_parser(token_group_idx);
        Ok(Ast::Defn {
            // order matters!
            accessibility: aux_parser.parse_accessibility()?,
            entity_card: aux_parser.parse_entity_card()?,
            ident: aux_parser.parse_ident()?,
            is_generic: aux_parser.parse_is_generic(),
            token_group_idx,
            body,
            body_kind,
        })
    }
}
