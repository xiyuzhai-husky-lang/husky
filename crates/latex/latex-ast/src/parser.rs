use crate::ast::{LxAstArena, LxAstData, LxAstIdx, LxAstIdxRange};
use latex_annotation::{
    annotation::{space::LxSpaceAnnotation, token::LxTokenAnnotation},
    annotations::LxAnnotations,
};
use latex_prelude::mode::LxMode;
use latex_token::{data::LxTokenData, idx::LxTokenIdx, lexer::LxLexer};
use std::iter::Peekable;

pub(crate) struct LxAstParser<'a> {
    db: &'a ::salsa::Db,
    lexer: Peekable<LxLexer<'a>>,
    annotations: &'a LxAnnotations,
    arena: &'a mut LxAstArena,
}

/// # constructor
impl<'a> LxAstParser<'a> {
    pub(crate) fn new(
        db: &'a ::salsa::Db,
        input: &'a str,
        annotations: &'a LxAnnotations,
        mode: LxMode,
        arena: &'a mut LxAstArena,
    ) -> Self {
        Self {
            db,
            lexer: LxLexer::new(db, input, mode).peekable(),
            annotations,
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
        self.lexer.peek().map(|&(_, _, data)| data)
    }

    pub(crate) fn next_token(
        &mut self,
    ) -> Option<(
        LxTokenIdx,
        LxTokenData,
        LxTokenAnnotation,
        LxSpaceAnnotation,
    )> {
        let (token_idx, range, token_data) = self.lexer.next()?;
        let token_annotation = todo!();
        let space_annotation = todo!();
        Some((token_idx, token_data, token_annotation, space_annotation))
    }
}
