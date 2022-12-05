use crate::*;
use husky_token::{Keyword, SpecialToken, TokenGroupIter, TokenKind, TokenSheet};

pub(crate) struct AstParser<'a> {
    db: &'a dyn WordDb,
    arena: AstArena,
    token_groups: TokenGroupIter<'a>,
}

impl<'a> AstParser<'a> {
    pub(crate) fn new(db: &'a dyn WordDb, token_sheet: &'a TokenSheet) -> Self {
        Self {
            db,
            arena: Default::default(),
            token_groups: token_sheet.token_groups(),
        }
    }

    pub(crate) fn parse_all(mut self) -> AstSheet {
        let top_level_asts = self.parse_asts_above(0);
        AstSheet::new(self.arena, top_level_asts)
    }

    fn parse_asts_above(&mut self, indent: u32) -> AstIdxRange {
        let mut asts: Vec<Ast> = vec![];
        while let Some(ast) = self.parse_ast_above(indent) {
            asts.push(ast)
        }
        self.arena.alloc_batch(asts)
    }

    fn parse_ast_above(&mut self, indent: u32) -> Option<Ast> {
        let (idx, token_group) = self.token_groups.next_above(indent)?;
        Some(match token_group.first().kind {
            TokenKind::Decorator(_) => todo!(),
            TokenKind::Keyword(kw) => match kw {
                Keyword::Config(_) => todo!(),
                Keyword::Paradigm(_) => todo!(),
                Keyword::Type(_) => Ast::Defn {
                    head: idx,
                    body: self.parse_asts_above(indent + 4),
                },
                Keyword::Stmt(_) => todo!(),
                Keyword::Liason(_) => todo!(),
                Keyword::Ambiguous(_) => todo!(),
                Keyword::Main => todo!(),
                Keyword::Use => Ast::Use(idx),
                Keyword::Mod => Ast::Mod(idx),
                Keyword::Visual => todo!(),
                Keyword::Impl => todo!(),
                Keyword::End(_) => todo!(),
            },
            TokenKind::Special(SpecialToken::PoundSign) => Ast::Decor(idx),
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
        })
    }
}
