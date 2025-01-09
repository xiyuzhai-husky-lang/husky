use super::{
    chunk::VdMirHypothesisChunk, construction::VdMirHypothesisConstruction, VdMirHypothesisArena,
    VdMirHypothesisIdx,
};
use crate::{
    expr::{VdMirExprArena, VdMirExprArenaRef, VdMirExprData, VdMirExprEntry, VdMirExprIdx},
    hint::VdMirHintArena,
    hypothesis::{VdMirHypothesisEntry, VdMirHypothesisIdxRange},
    region::VdMirExprRegionDataRef,
    stmt::{VdMirStmtArena, VdMirStmtArenaRef, VdMirStmtIdx},
    symbol::local_defn::storage::VdMirSymbolLocalDefnStorage,
};
use eterned::db::EternerDb;
use rustc_hash::FxHashMap;
use visored_term::ty::VdType;

pub struct VdMirHypothesisConstructor<'db, Src> {
    db: &'db EternerDb,
    expr_arena: VdMirExprArena,
    stmt_arena: VdMirStmtArena,
    hint_arena: VdMirHintArena,
    hypothesis_arena: VdMirHypothesisArena,
    symbol_local_defn_storage: VdMirSymbolLocalDefnStorage,
    current_stmt_and_hypothesis_chunk_start: Option<(VdMirStmtIdx, VdMirHypothesisIdx)>,
    cache: FxHashMap<Src, VdMirHypothesisIdx>,
}

impl<'db, Src> VdMirHypothesisConstructor<'db, Src> {
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
            cache: FxHashMap::default(),
        }
    }
}

impl<'db, Src> VdMirHypothesisConstructor<'db, Src> {
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

impl<'db, Src> VdMirHypothesisConstructor<'db, Src> {
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
        src: Src,
        f: impl Fn(&mut Self) -> (VdMirExprIdx, VdMirHypothesisConstruction),
    ) -> VdMirHypothesisIdx
    where
        Src: std::hash::Hash + Eq,
    {
        assert!(self.current_stmt_and_hypothesis_chunk_start.is_some());
        if let Some(&hypothesis) = self.cache.get(&src) {
            return hypothesis;
        }
        let (expr, hypothesis) = f(self);
        let hypothesis = self
            .hypothesis_arena
            .alloc_one(VdMirHypothesisEntry::new(expr, hypothesis));
        self.cache.insert(src, hypothesis);
        hypothesis
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
