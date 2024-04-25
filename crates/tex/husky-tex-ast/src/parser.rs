use crate::data::{TexAstArena, TexAstData, TexAstIdx, TexAstIdxRange};
use husky_tex_prelude::mode::TexMode;
use husky_tex_token::{data::TexTokenData, idx::TexTokenIdx, lexer::TexLexer};
use std::iter::Peekable;

pub(crate) struct TexAstParser<'a> {
    db: &'a ::salsa::Db,
    lexer: Peekable<TexLexer<'a>>,
    arena: &'a mut TexAstArena,
}

/// # constructor
impl<'a> TexAstParser<'a> {
    pub(crate) fn new(
        db: &'a ::salsa::Db,
        input: &'a str,
        mode: TexMode,
        arena: &'a mut TexAstArena,
    ) -> Self {
        Self {
            db,
            lexer: TexLexer::new(db, input, mode).peekable(),
            arena,
        }
    }

    pub(crate) fn alloc_asts(&mut self, asts: Vec<TexAstData>) -> TexAstIdxRange {
        self.arena.alloc_batch(asts)
    }

    pub(crate) fn alloc_ast(&mut self, ast: TexAstData) -> TexAstIdx {
        self.arena.alloc_one(ast)
    }
}

/// # actions
impl<'a> TexAstParser<'a> {
    pub(crate) fn peek_token(&mut self) -> Option<TexTokenData> {
        self.lexer.peek().map(|&(_, data)| data)
    }

    pub(crate) fn next_token(&mut self) -> Option<(TexTokenIdx, TexTokenData)> {
        self.lexer.next()
    }
}
