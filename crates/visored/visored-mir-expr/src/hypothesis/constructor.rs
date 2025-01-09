use crate::{
    expr::{VdMirExprArena, VdMirExprArenaRef, VdMirExprData, VdMirExprEntry, VdMirExprIdx},
    hint::VdMirHintArena,
    hypothesis::{VdMirHypothesisEntry, VdMirHypothesisIdxRange},
    region::VdMirExprRegionDataRef,
    stmt::{VdMirStmtArena, VdMirStmtArenaRef, VdMirStmtIdx},
    symbol::local_defn::storage::VdMirSymbolLocalDefnStorage,
};
use eterned::db::EternerDb;
use visored_term::ty::VdType;

use super::{
    chunk::VdMirHypothesisChunk, construction::VdMirHypothesisConstruction, VdMirHypothesisArena,
    VdMirHypothesisIdx,
};

pub struct VdMirHypothesisConstructor<'db> {
    db: &'db EternerDb,
    expr_arena: VdMirExprArena,
    stmt_arena: VdMirStmtArena,
    hint_arena: VdMirHintArena,
    hypothesis_arena: VdMirHypothesisArena,
    symbol_local_defn_storage: VdMirSymbolLocalDefnStorage,
    current_stmt_and_hypothesis_chunk_start: Option<(VdMirStmtIdx, VdMirHypothesisIdx)>,
}

impl<'db> VdMirHypothesisConstructor<'db> {
    pub fn new(
        db: &'db EternerDb,
        expr_arena: VdMirExprArena,
        stmt_arena: VdMirStmtArena,
        hint_arena: VdMirHintArena,
        symbol_local_defn_storage: VdMirSymbolLocalDefnStorage,
    ) -> Self {
        Self {
            db,
            expr_arena,
            stmt_arena,
            hint_arena,
            symbol_local_defn_storage,
            hypothesis_arena: Default::default(),
            current_stmt_and_hypothesis_chunk_start: None,
        }
    }
}

impl<'db> VdMirHypothesisConstructor<'db> {
    pub fn expr_arena(&self) -> VdMirExprArenaRef {
        self.expr_arena.as_arena_ref()
    }

    pub fn stmt_arena(&self) -> VdMirStmtArenaRef {
        self.stmt_arena.as_arena_ref()
    }

    pub fn stmt_arena_mut(&mut self) -> &mut VdMirStmtArena {
        &mut self.stmt_arena
    }

    pub fn region_data(&self) -> VdMirExprRegionDataRef {
        VdMirExprRegionDataRef {
            expr_arena: self.expr_arena.as_arena_ref(),
            stmt_arena: self.stmt_arena.as_arena_ref(),
            hint_arena: self.hint_arena.as_arena_ref(),
            symbol_local_defn_storage: &self.symbol_local_defn_storage,
        }
    }
}

impl<'db> VdMirHypothesisConstructor<'db> {
    pub(crate) fn obtain_hypothesis_chunk_within_stmt(
        &mut self,
        stmt: VdMirStmtIdx,
        f: impl FnOnce(&mut Self) -> VdMirHypothesisIdx,
    ) -> VdMirHypothesisChunk {
        assert!(self.current_stmt_and_hypothesis_chunk_start.is_none());
        self.current_stmt_and_hypothesis_chunk_start = Some((stmt, unsafe {
            VdMirHypothesisIdx::new_ext(self.hypothesis_arena.len())
        }));
        let result = f(self);
        let Some((stmt, chunk_start)) = self.current_stmt_and_hypothesis_chunk_start else {
            unreachable!()
        };
        self.current_stmt_and_hypothesis_chunk_start = None;
        VdMirHypothesisChunk::new(
            unsafe {
                VdMirHypothesisIdxRange::new(chunk_start, unsafe {
                    VdMirHypothesisIdx::new_ext(self.hypothesis_arena.len())
                })
            },
            result,
        )
    }

    // TODO: do more things like handle hypothesis stack, register src, etc.
    pub fn construct_new_hypothesis(
        &mut self,
        expr: VdMirExprIdx,
        hypothesis: VdMirHypothesisConstruction,
    ) -> VdMirHypothesisIdx {
        assert!(self.current_stmt_and_hypothesis_chunk_start.is_some());
        self.hypothesis_arena
            .alloc_one(VdMirHypothesisEntry::new(expr, hypothesis))
    }

    pub fn construct_new_expr(
        &mut self,
        data: VdMirExprData,
        ty: VdType,
        expected_ty: Option<VdType>,
    ) -> VdMirExprIdx {
        self.expr_arena
            .alloc_one(VdMirExprEntry::new(data, ty, expected_ty))
    }

    pub(crate) fn finish(
        self,
    ) -> (
        VdMirExprArena,
        VdMirStmtArena,
        VdMirHintArena,
        VdMirHypothesisArena,
        VdMirSymbolLocalDefnStorage,
    ) {
        (
            self.expr_arena,
            self.stmt_arena,
            self.hint_arena,
            self.hypothesis_arena,
            self.symbol_local_defn_storage,
        )
    }
}
