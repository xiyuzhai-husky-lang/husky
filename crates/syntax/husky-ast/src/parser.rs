use crate::*;
use husky_token::{TokenGroupIter, TokenKind, TokenSheet};

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
        let asts: Vec<Ast> = vec![];
        while let Some((idx, token_group)) = self.token_groups.next() {
            if token_group.indent() != indent {
                todo!()
            }
            match token_group.first().kind {
                TokenKind::Decorator(_) => todo!(),
                TokenKind::Keyword(_) => todo!(),
                TokenKind::Identifier(ident) => {
                    p!(self.db.dt_ident(ident));
                    todo!()
                }
                TokenKind::Special(_) => todo!(),
                TokenKind::WordOpr(_) => todo!(),
                TokenKind::Literal(_) => todo!(),
                TokenKind::Unrecognized(_) => todo!(),
                TokenKind::IllFormedLiteral(_) => todo!(),
            }
            todo!()
        }
        self.arena.alloc_batch(asts)
    }
}
