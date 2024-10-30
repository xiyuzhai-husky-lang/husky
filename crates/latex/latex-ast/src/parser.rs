use crate::{
    ast::{LxAstArena, LxAstData, LxAstIdx, LxAstIdxRange},
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
    pub(crate) fn alloc_asts(&mut self, asts: Vec<LxAstData>) -> LxAstIdxRange {
        self.arena.alloc_batch(asts)
    }

    pub(crate) fn alloc_ast(&mut self, ast: LxAstData) -> LxAstIdx {
        self.arena.alloc_one(ast)
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
