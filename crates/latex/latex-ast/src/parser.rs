use crate::{
    ast::{
        math::{LxMathAstData, LxMathAstIdx, LxMathAstIdxRange},
        rose::{LxRoseAstData, LxRoseAstIdx, LxRoseAstIdxRange},
        LxAstArena, LxAstData, LxAstIdx, LxAstIdxRange,
    },
    region::LxAstRegionData,
};
use latex_annotation::{
    annotation::{space::LxSpaceAnnotation, token::LxTokenAnnotation},
    annotations::{walker::LxAnnotationsWalker, LxAnnotations},
};
use latex_prelude::mode::LxMode;
use latex_token::{
    data::LxTokenData, idx::LxTokenIdx, storage::LxTokenStorage, stream::LxTokenStream,
};
use std::{borrow::BorrowMut, iter::Peekable};

pub(crate) struct LxAstParser<'a> {
    db: &'a ::salsa::Db,
    token_stream: Peekable<LxTokenStream<'a>>,
    annotations_walker: LxAnnotationsWalker<'a>,
    arena: &'a mut LxAstArena,
}

/// # constructor
impl<'a> LxAstParser<'a> {
    pub(crate) fn new(
        db: &'a ::salsa::Db,
        tokens: &'a LxTokenStorage,
        annotations: &'a LxAnnotations,
        mode: LxMode,
        arena: &'a mut LxAstArena,
    ) -> Self {
        Self {
            db,
            token_stream: tokens.stream().peekable(),
            annotations_walker: annotations.walker(),
            arena,
        }
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

    pub(crate) fn peek_token(&mut self) -> Option<LxTokenData> {
        self.token_stream.peek().map(|&(_, _, _, data)| data)
    }

    pub(crate) fn next_token(
        &mut self,
    ) -> Option<(
        LxTokenIdx,
        LxTokenData,
        LxTokenAnnotation,
        LxSpaceAnnotation,
    )> {
        let (token_idx, (start, end), _, token_data) = self.token_stream.next()?;
        let (token_annotation, space_annotation) = self.annotations_walker.next(start, end);
        Some((token_idx, token_data, token_annotation, space_annotation))
    }
}
