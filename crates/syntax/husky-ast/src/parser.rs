use crate::*;
use husky_token::{Keyword, SpecialToken, TokenGroupIter, TokenKind, TokenSheet};

pub(crate) struct AstParser<'a> {
    db: &'a dyn WordDb,
    arena: AstArena,
    token_groups: TokenGroupIter<'a>,
    token_group_map: Vec<TokenGroupIdx>,
}

impl<'a> AstParser<'a> {
    pub(crate) fn new(db: &'a dyn WordDb, token_sheet: &'a TokenSheet) -> Self {
        Self {
            db,
            arena: Default::default(),
            token_groups: token_sheet.token_groups(),
            token_group_map: vec![],
        }
    }

    pub(crate) fn parse_all(mut self) -> AstSheet {
        let top_level_asts = self.parse_asts(0);
        AstSheet::new(self.arena, top_level_asts)
    }

    fn parse_asts(&mut self, indent: u32) -> AstIdxRange {
        let mut asts: Vec<Ast> = vec![];
        let mut token_group_indices: Vec<TokenGroupIdx> = vec![];
        while let Some((idx, ast)) = self.parse_ast(indent) {
            token_group_indices.push(idx);
            asts.push(ast)
        }
        self.alloc_asts(asts, token_group_indices)
    }

    fn alloc_asts(
        &mut self,
        asts: Vec<Ast>,
        token_group_indices: Vec<TokenGroupIdx>,
    ) -> AstIdxRange {
        self.token_group_map.extend(token_group_indices);
        self.arena.alloc_batch(asts)
    }

    fn parse_ast(&mut self, indent: u32) -> Option<(TokenGroupIdx, Ast)> {
        let (idx, token_group) = self.token_groups.next_indented(indent)?;
        Some((
            idx,
            if token_group.indent() > indent {
                Ast::ExcessiveIndent
            } else {
                match token_group.first().kind {
                    TokenKind::Decorator(_) => todo!(),
                    TokenKind::Keyword(kw) => match kw {
                        Keyword::Stmt(_) => todo!(),
                        Keyword::Liason(_) => todo!(),
                        Keyword::Main => todo!(),
                        Keyword::Use => Ast::Use,
                        Keyword::Mod => Ast::Mod,
                        Keyword::Config(_)
                        | Keyword::Paradigm(_)
                        | Keyword::Visual
                        | Keyword::Type(_)
                        | Keyword::Impl => Ast::Defn(self.parse_asts(indent + 4)),
                        Keyword::End(_) => todo!(),
                    },
                    TokenKind::Special(SpecialToken::PoundSign) => Ast::Decor,
                    TokenKind::Identifier(ident) => {
                        p!(self.db.dt_ident(ident));
                        todo!()
                    }
                    TokenKind::Special(_) => todo!(),
                    TokenKind::WordOpr(_) => todo!(),
                    TokenKind::Literal(_) => todo!(),
                    TokenKind::Unrecognized(c) => {
                        p!(c);
                        todo!()
                    }
                    TokenKind::IllFormedLiteral(_) => todo!(),
                }
            },
        ))
    }
}
