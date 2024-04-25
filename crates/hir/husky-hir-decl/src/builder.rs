use husky_eth_term::term::EthTerm;
use husky_fly_term::FlyTermBase;
use husky_hir_eager_expr::{HirEagerExprIdx, HirEagerPatternIdx};
use husky_hir_expr::{
    helpers::hir_expr_region_with_source_map, source_map::HirExprSourceMap, HirExprRegion,
};
use husky_hir_lazy_expr::HirLazyPatternExprIdx;
use husky_hir_ty::{db::HirTypeDb, menu::HirTypeMenu, trai::HirTrait, HirType};
use husky_sem_expr::{SemaExprDb, SemaExprRegionData};
use husky_syn_expr::{
    CurrentVariableIdx, ReturnTypeBeforeColonSyndicate, ReturnTypeBeforeEqSyndicate, SynExprIdx,
    SynExprRegion, SynExprRegionData, SynPatternRoot,
};

pub(crate) struct HirDeclBuilder<'a> {
    db: &'a ::salsa::Db,
    hir_ty_menu: &'a HirTypeMenu,
    syn_expr_region_data: &'a SynExprRegionData,
    sem_expr_region_data: &'a SemaExprRegionData,
    hir_expr_region: HirExprRegion,
    hir_expr_source_map: HirExprSourceMap,
}

impl<'a> HirDeclBuilder<'a> {
    pub(crate) fn new(syn_expr_region: SynExprRegion, db: &'a ::salsa::Db) -> Self {
        let toolchain = syn_expr_region.data(db).path().toolchain(db);
        let hir_ty_menu = db.hir_ty_menu(toolchain);
        let (hir_expr_region, hir_expr_source_map) =
            hir_expr_region_with_source_map(syn_expr_region, db);
        let sem_expr_region = db.sem_expr_region(syn_expr_region);
        let syn_expr_region = sem_expr_region.syn_expr_region(db);
        Self {
            db,
            hir_ty_menu,
            syn_expr_region_data: syn_expr_region.data(db),
            sem_expr_region_data: sem_expr_region.data(db),
            hir_expr_region,
            hir_expr_source_map,
        }
    }

    pub(crate) fn return_ty_before_eq(
        &self,
        return_ty_syndicate: impl Into<Option<ReturnTypeBeforeEqSyndicate>>,
    ) -> HirType {
        return_ty_syndicate
            .into()
            .map(|syndicate| self.hir_ty(syndicate.syn_expr_idx()).unwrap())
            .unwrap_or(self.hir_ty_menu.unit_ty().into())
    }

    pub(crate) fn return_ty_before_colon(
        &self,
        return_ty_syndicate: Option<ReturnTypeBeforeColonSyndicate>,
    ) -> HirType {
        return_ty_syndicate
            .map(|syndicate| self.hir_ty(syndicate.syn_expr_idx()).unwrap())
            .unwrap_or(self.hir_ty_menu.unit_ty().into())
    }

    pub(crate) fn hir_ty(&self, syn_expr_idx: SynExprIdx) -> Option<HirType> {
        let sem_expr_idx = self
            .sem_expr_region_data
            .syn_root_to_sem_expr_idx(syn_expr_idx);
        match self
            .sem_expr_region_data
            .sem_expr_term(sem_expr_idx)
            .unwrap()
            .unwrap()
            .base_resolved_inner(self.sem_expr_region_data.fly_term_region().hollow_terms())
        {
            FlyTermBase::Eth(term) => HirType::from_eth(term, self.db),
            FlyTermBase::Sol(_) => todo!(),
            FlyTermBase::Hol(_) => todo!(),
            FlyTermBase::Place => todo!(),
        }
    }

    pub(crate) fn hir_trai(&self, syn_expr_idx: SynExprIdx) -> HirTrait {
        let sem_expr_idx = self
            .sem_expr_region_data
            .syn_root_to_sem_expr_idx(syn_expr_idx);
        match self
            .sem_expr_region_data
            .sem_expr_term(sem_expr_idx)
            .unwrap()
            .unwrap()
            .base_resolved_inner(self.sem_expr_region_data.fly_term_region().hollow_terms())
        {
            FlyTermBase::Eth(term) => HirTrait::from_eth(term, self.db),
            FlyTermBase::Sol(_) => todo!(),
            FlyTermBase::Hol(_) => todo!(),
            FlyTermBase::Place => todo!(),
        }
    }

    pub(crate) fn finish(self) -> HirExprRegion {
        self.hir_expr_region
    }

    pub(crate) fn current_variable_term(
        &self,
        current_variable_idx: CurrentVariableIdx,
    ) -> EthTerm {
        match self.sem_expr_region_data.symbol_terms()[current_variable_idx]
            .base_resolved_inner(self.sem_expr_region_data.fly_term_region())
        {
            FlyTermBase::Eth(symbol_term) => symbol_term,
            FlyTermBase::Sol(_) => todo!(),
            FlyTermBase::Hol(_) => todo!(),
            FlyTermBase::Place => todo!(),
        }
    }

    pub(crate) fn db(&self) -> &::salsa::Db {
        self.db
    }

    pub(crate) fn hir_eager_expr_idx(&self, syn_expr_root: SynExprIdx) -> Option<HirEagerExprIdx> {
        let HirExprSourceMap::Eager(source_map) = self.hir_expr_source_map else {
            unreachable!()
        };
        let sem_expr_idx = self
            .sem_expr_region_data
            .syn_root_to_sem_expr_idx(syn_expr_root);
        source_map
            .data(self.db)
            .sem_to_hir_eager_expr_idx(sem_expr_idx)
    }

    pub(crate) fn hir_eager_pattern_expr_idx(
        &self,
        syn_pattern_root: impl Into<SynPatternRoot>,
    ) -> HirEagerPatternIdx {
        let HirExprSourceMap::Eager(source_map) = self.hir_expr_source_map else {
            unreachable!()
        };
        let db = self.db;
        source_map
            .data(db)
            .syn_pattern_root_to_sem_expr_idx(syn_pattern_root)
    }

    pub(crate) fn hir_lazy_pattern_expr_idx(
        &self,
        syn_pattern_root: impl Into<SynPatternRoot>,
    ) -> HirLazyPatternExprIdx {
        let HirExprSourceMap::Lazy(source_map) = self.hir_expr_source_map else {
            unreachable!()
        };
        let db = self.db;
        source_map
            .data(db)
            .syn_pattern_root_to_sem_expr_idx(syn_pattern_root)
    }

    pub(crate) fn syn_expr_region_data(&self) -> &'a SynExprRegionData {
        self.syn_expr_region_data
    }

    pub(crate) fn sem_expr_region_data(&self) -> &'a SemaExprRegionData {
        self.sem_expr_region_data
    }
}
