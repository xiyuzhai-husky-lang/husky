use crate::ast::{LxAstArena, LxAstData, LxAstIdx, LxAstIdxRange};
use latex_prelude::mode::LxMode;
use latex_token::{data::LxTokenData, idx::LxTokenIdx, lexer::LxLexer};
use std::iter::Peekable;

pub(crate) struct LxAstParser<'a> {
    db: &'a ::salsa::Db,
    lexer: Peekable<LxLexer<'a>>,
    arena: &'a mut LxAstArena,
}

/// # constructor
impl<'a> LxAstParser<'a> {
    pub(crate) fn new(
        db: &'a ::salsa::Db,
        input: &'a str,
        mode: LxMode,
        arena: &'a mut LxAstArena,
    ) -> Self {
        Self {
            db,
            lexer: LxLexer::new(db, input, mode).peekable(),
            arena,
        }
    }

    pub(crate) fn alloc_asts(&mut self, asts: Vec<LxAstData>) -> LxAstIdxRange {
        self.arena.alloc_batch(asts)
    }

    pub(crate) fn alloc_ast(&mut self, ast: LxAstData) -> LxAstIdx {
        self.arena.alloc_one(ast)
    }
}

/// # actions
impl<'a> LxAstParser<'a> {
    pub(crate) fn peek_token(&mut self) -> Option<LxTokenData> {
        self.lexer.peek().map(|&(_, data)| data)
    }

    pub(crate) fn next_token(&mut self) -> Option<(LxTokenIdx, LxTokenData)> {
        self.lexer.next()
    }
}
