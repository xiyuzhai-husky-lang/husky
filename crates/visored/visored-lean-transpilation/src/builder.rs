use latex_token::storage::LxTokenStorage;
use lean_coword::ident::LnIdent;
use lean_mir_expr::{
    constructor::{LnMirExprConstructor, WithLnNamespace},
    expr::{LnMirExprArena, LnMirExprData},
    item_defn::{def::LnMirDefBody, LnItemDefnArena, LnItemDefnData, LnItemDefnIdxRange},
    stmt::LnMirStmtArena,
    tactic::LnMirTacticArena,
};
use std::ops::{Deref, DerefMut};
use visored_entity_path::module::VdModulePath;
use visored_mir_expr::{
    expr::VdMirExprArenaRef,
    region::VdMirExprRegionData,
    source_map::VdMirSourceMap,
    stmt::VdMirStmtArenaRef,
    symbol::local_defn::{storage::VdMirSymbolLocalDefnStorage, VdMirSymbolLocalDefnIdx},
};
use visored_sem_expr::range::{
    VdSemClauseTokenIdxRangeMap, VdSemDivisionTokenIdxRangeMap, VdSemExprTokenIdxRangeMap,
    VdSemPhraseTokenIdxRangeMap, VdSemSentenceTokenIdxRangeMap, VdSemStmtTokenIdxRangeMap,
};

use crate::{
    dictionary::VdLeanDictionary,
    mangle::VdLeanTranspilationMangler,
    namespace::{vd_module_path_to_ln_namespace, vd_module_path_to_ln_namespace_or_inherited},
};

pub struct VdLeanTranspilationBuilder<'a> {
    lean_hir_expr_builder: LnMirExprConstructor,
    expr_arena: VdMirExprArenaRef<'a>,
    stmt_arena: VdMirStmtArenaRef<'a>,
    dictionary: &'a VdLeanDictionary,
    mangler: VdLeanTranspilationMangler,
    current_module_path: VdModulePath,
    source_map: &'a VdMirSourceMap,
    sem_expr_range_map: &'a VdSemExprTokenIdxRangeMap,
    sem_phrase_range_map: &'a VdSemPhraseTokenIdxRangeMap,
    sem_clause_range_map: &'a VdSemClauseTokenIdxRangeMap,
    sem_sentence_range_map: &'a VdSemSentenceTokenIdxRangeMap,
    sem_stmt_range_map: &'a VdSemStmtTokenIdxRangeMap,
    sem_division_range_map: &'a VdSemDivisionTokenIdxRangeMap,
    token_storage: &'a LxTokenStorage,
    input: &'a str,
}

impl<'a> WithLnNamespace for VdLeanTranspilationBuilder<'a> {
    fn ln_mir_expr_builder_mut(&mut self) -> &mut LnMirExprConstructor {
        &mut self.lean_hir_expr_builder
    }
}

impl<'a> VdLeanTranspilationBuilder<'a> {
    pub fn new0(
        input: &'a str,
        vd_mir_expr_region_data: &'a VdMirExprRegionData,
        source_map: &'a VdMirSourceMap,
        dictionary: &'a VdLeanDictionary,
        root_module_path: VdModulePath,
        sem_expr_range_map: &'a VdSemExprTokenIdxRangeMap,
        sem_phrase_range_map: &'a VdSemPhraseTokenIdxRangeMap,
        sem_clause_range_map: &'a VdSemClauseTokenIdxRangeMap,
        sem_sentence_range_map: &'a VdSemSentenceTokenIdxRangeMap,
        sem_stmt_range_map: &'a VdSemStmtTokenIdxRangeMap,
        sem_division_range_map: &'a VdSemDivisionTokenIdxRangeMap,
        token_storage: &'a LxTokenStorage,
    ) -> Self {
        Self::new(
            input,
            vd_mir_expr_region_data.expr_arena(),
            vd_mir_expr_region_data.stmt_arena(),
            vd_mir_expr_region_data.symbol_local_defn_storage(),
            source_map,
            dictionary,
            root_module_path,
            sem_expr_range_map,
            sem_phrase_range_map,
            sem_clause_range_map,
            sem_sentence_range_map,
            sem_stmt_range_map,
            sem_division_range_map,
            token_storage,
        )
    }

    pub fn new(
        input: &'a str,
        expr_arena: VdMirExprArenaRef<'a>,
        stmt_arena: VdMirStmtArenaRef<'a>,
        symbol_local_defn_storage: &'a VdMirSymbolLocalDefnStorage,
        source_map: &'a VdMirSourceMap,
        dictionary: &'a VdLeanDictionary,
        root_module_path: VdModulePath,
        sem_expr_range_map: &'a VdSemExprTokenIdxRangeMap,
        sem_phrase_range_map: &'a VdSemPhraseTokenIdxRangeMap,
        sem_clause_range_map: &'a VdSemClauseTokenIdxRangeMap,
        sem_sentence_range_map: &'a VdSemSentenceTokenIdxRangeMap,
        sem_stmt_range_map: &'a VdSemStmtTokenIdxRangeMap,
        sem_division_range_map: &'a VdSemDivisionTokenIdxRangeMap,
        token_storage: &'a LxTokenStorage,
    ) -> Self {
        Self {
            lean_hir_expr_builder: LnMirExprConstructor::new(),
            expr_arena,
            stmt_arena,
            source_map,
            dictionary,
            mangler: VdLeanTranspilationMangler::new(symbol_local_defn_storage),
            current_module_path: root_module_path,
            sem_expr_range_map,
            sem_phrase_range_map,
            sem_clause_range_map,
            sem_sentence_range_map,
            sem_stmt_range_map,
            sem_division_range_map,
            token_storage,
            input,
        }
    }

    pub(crate) fn with_module_path<R>(
        &mut self,
        module_path: VdModulePath,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        debug_assert_eq!(
            module_path.parent(),
            Some(self.current_module_path),
            "module path = {}, current module path = {}",
            module_path.show(),
            self.current_module_path.show(),
        );
        let namespace = *vd_module_path_to_ln_namespace(module_path);
        let prev_module_path = self.current_module_path;
        self.current_module_path = module_path;
        let result = if let Some(namespace) = namespace {
            self.with_ln_namespace(namespace, f)
        } else {
            f(self)
        };
        self.current_module_path = prev_module_path;
        result
    }

    pub(crate) fn current_module_path(&self) -> VdModulePath {
        self.current_module_path
    }

    pub(crate) fn mangle_symbol(&mut self, symbol_local_defn: VdMirSymbolLocalDefnIdx) -> LnIdent {
        self.mangler.mangle_symbol(symbol_local_defn)
    }

    pub(crate) fn mangle_hypothesis(&mut self) -> LnIdent {
        self.mangler
            .mangle_hypothesis(*vd_module_path_to_ln_namespace_or_inherited(
                self.current_module_path(),
            ))
    }

    pub(crate) fn sorry(&mut self) -> LnMirDefBody {
        LnMirDefBody::Expr(self.alloc_expr(LnMirExprData::Sorry))
    }
}

impl<'db> VdLeanTranspilationBuilder<'db> {
    pub fn expr_arena(&self) -> VdMirExprArenaRef<'db> {
        self.expr_arena
    }

    pub fn stmt_arena(&self) -> VdMirStmtArenaRef<'db> {
        self.stmt_arena
    }

    pub fn source_map(&self) -> &'db VdMirSourceMap {
        self.source_map
    }

    pub fn dictionary(&self) -> &'db VdLeanDictionary {
        self.dictionary
    }

    pub fn input(&self) -> &'db str {
        self.input
    }

    pub fn token_storage(&self) -> &'db LxTokenStorage {
        self.token_storage
    }

    pub fn sem_expr_range_map(&self) -> &'db VdSemExprTokenIdxRangeMap {
        self.sem_expr_range_map
    }

    pub fn sem_phrase_range_map(&self) -> &'db VdSemPhraseTokenIdxRangeMap {
        self.sem_phrase_range_map
    }

    pub fn sem_clause_range_map(&self) -> &'db VdSemClauseTokenIdxRangeMap {
        self.sem_clause_range_map
    }

    pub fn sem_sentence_range_map(&self) -> &'db VdSemSentenceTokenIdxRangeMap {
        self.sem_sentence_range_map
    }

    pub fn sem_stmt_range_map(&self) -> &'db VdSemStmtTokenIdxRangeMap {
        self.sem_stmt_range_map
    }

    pub fn sem_division_range_map(&self) -> &'db VdSemDivisionTokenIdxRangeMap {
        self.sem_division_range_map
    }
}

impl<'db> Deref for VdLeanTranspilationBuilder<'db> {
    type Target = LnMirExprConstructor;

    fn deref(&self) -> &Self::Target {
        &self.lean_hir_expr_builder
    }
}

impl<'db> DerefMut for VdLeanTranspilationBuilder<'db> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lean_hir_expr_builder
    }
}

impl<'db> VdLeanTranspilationBuilder<'db> {
    pub fn finish(
        self,
    ) -> (
        LnMirExprArena,
        LnMirStmtArena,
        LnMirTacticArena,
        LnItemDefnArena,
    ) {
        self.lean_hir_expr_builder.finish()
    }
}
