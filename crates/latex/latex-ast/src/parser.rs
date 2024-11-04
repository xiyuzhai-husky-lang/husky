use crate::{
    ast::{
        math::{LxMathAstData, LxMathAstIdx, LxMathAstIdxRange},
        rose::{LxRoseAstData, LxRoseAstIdx, LxRoseAstIdxRange},
        LxAstArena, LxAstData, LxAstIdx, LxAstIdxRange,
    },
    region::LxAstRegionData,
};
use latex_command::signature::table::LxCommandSignatureTable;
use latex_prelude::mode::LxMode;
use latex_token::{
    data::{math::LxMathTokenData, rose::LxRoseTokenData},
    idx::{LxMathTokenIdx, LxRoseTokenIdx},
    lexer::LxLexer,
    storage::LxTokenStorage,
};
use std::{borrow::BorrowMut, iter::Peekable};

pub(crate) struct LxAstParser<'a> {
    db: &'a ::salsa::Db,
    command_signature_table: &'a LxCommandSignatureTable,
    lexer: LxLexer<'a>,
    mode: LxMode,
    arena: &'a mut LxAstArena,
}

/// # constructor
impl<'a> LxAstParser<'a> {
    pub(crate) fn new(
        db: &'a ::salsa::Db,
        command_signature_table: &'a LxCommandSignatureTable,
        input: &'a str,
        mode: LxMode,
        token_storage: &'a mut LxTokenStorage,
        arena: &'a mut LxAstArena,
    ) -> Self {
        Self {
            db,
            command_signature_table,
            lexer: LxLexer::new(db, input, token_storage),
            mode,
            arena,
        }
    }
}

impl<'a> LxAstParser<'a> {
    pub(crate) fn mode(&self) -> LxMode {
        self.mode
    }

    pub(crate) fn command_signature_table(&self) -> &LxCommandSignatureTable {
        self.command_signature_table
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

    pub(crate) fn peek_math_token_data(&mut self) -> Option<LxMathTokenData> {
        self.lexer.peek_math_token_data()
    }

    pub(crate) fn peek_rose_token_data(&mut self) -> Option<LxRoseTokenData> {
        self.lexer.peek_rose_token_data()
    }

    pub(crate) fn next_math_token(&mut self) -> Option<(LxMathTokenIdx, LxMathTokenData)> {
        self.lexer.next_math_token()
    }

    pub(crate) fn next_rose_token(&mut self) -> Option<(LxRoseTokenIdx, LxRoseTokenData)> {
        self.lexer.next_rose_token()
    }
}
