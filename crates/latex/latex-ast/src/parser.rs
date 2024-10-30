use crate::{
    ast::{
        math::{LxMathAstData, LxMathAstIdx, LxMathAstIdxRange},
        rose::{LxRoseAstData, LxRoseAstIdx, LxRoseAstIdxRange},
        LxAstArena, LxAstData, LxAstIdx, LxAstIdxRange,
    },
    region::LxAstRegionData,
};
use latex_prelude::mode::LxMode;
use latex_token::{
    data::LxTokenData, idx::LxTokenIdx, lexer::LxLexer, storage::LxTokenStorage,
    stream::LxTokenStream,
};
use std::{borrow::BorrowMut, iter::Peekable};

pub(crate) struct LxAstParser<'a> {
    db: &'a ::salsa::Db,
    lexer: LxLexer<'a>,
    arena: &'a mut LxAstArena,
}

/// # constructor
impl<'a> LxAstParser<'a> {
    pub(crate) fn new(
        db: &'a ::salsa::Db,
        input: &'a str,
        mode: LxMode,
        token_storage: &'a mut LxTokenStorage,
        arena: &'a mut LxAstArena,
    ) -> Self {
        Self {
            db,
            lexer: LxLexer::new(db, input, mode, token_storage),
            arena,
        }
    }

    pub(crate) fn mode(&self) -> LxMode {
        self.lexer.mode()
    }
}

/// # actions
impl<'a> LxAstParser<'a> {
    pub(crate) fn alloc_math_asts(&mut self, asts: Vec<LxMathAstData>) -> LxMathAstIdxRange {
        self.arena.math.alloc_batch(asts)
    }

    pub(crate) fn alloc_math_ast(&mut self, ast: LxMathAstData) -> LxMathAstIdx {
        self.arena.math.alloc_one(ast)
    }

    pub(crate) fn alloc_rose_ast(&mut self, ast: LxRoseAstData) -> LxRoseAstIdx {
        self.arena.rose.alloc_one(ast)
    }

    pub(crate) fn alloc_rose_asts(&mut self, asts: Vec<LxRoseAstData>) -> LxRoseAstIdxRange {
        self.arena.rose.alloc_batch(asts)
    }

    pub(crate) fn peek_char(&mut self) -> Option<char> {
        self.lexer.peek_char()
    }

    pub(crate) fn next_token(&mut self) -> Option<(LxTokenIdx, LxTokenData)> {
        let (token_idx, _, _, token_data) = self.lexer.next()?;
        Some((token_idx, token_data))
    }
}
