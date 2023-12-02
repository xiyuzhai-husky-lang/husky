use husky_ethereal_term::EtherealTerm;
use husky_fluffy_term::FluffyTermBase;
use husky_hir_eager_expr::HirEagerPatternExprIdx;
use husky_hir_expr::{
    helpers::hir_expr_region_with_source_map, source_map::HirExprSourceMap, HirExprRegion,
};
use husky_hir_lazy_expr::HirLazyPatternExprIdx;
use husky_hir_ty::{db::HirTypeDb, menu::HirTypeMenu, HirType};
use husky_sema_expr::{SemaExprDb, SemaExprRegionData};
use husky_syn_expr::{
    CurrentSynSymbolIdx, ReturnTypeBeforeColonSyndicate, ReturnTypeBeforeEqSyndicate, SynExprIdx,
    SynExprRegion, SynExprRegionData, SynPatternExprRoot,
};

pub(crate) struct HirDeclBuilder<'a> {
    db: &'a ::salsa::Db,
    hir_ty_menu: &'a HirTypeMenu,
    syn_expr_region_data: &'a SynExprRegionData,
    sema_expr_region_data: &'a SemaExprRegionData,
    hir_expr_region: HirExprRegion,
    hir_expr_source_map: HirExprSourceMap,
}

impl<'a> HirDeclBuilder<'a> {
    pub(crate) fn new(syn_expr_region: SynExprRegion, db: &'a ::salsa::Db) -> Self {
        let toolchain = syn_expr_region.data(db).path().toolchain(db);
        let hir_ty_menu = db.hir_ty_menu(toolchain);
        let (hir_expr_region, hir_expr_source_map) =
            hir_expr_region_with_source_map(syn_expr_region, db);
        let sema_expr_region = db.sema_expr_region(syn_expr_region);
        let syn_expr_region = sema_expr_region.syn_expr_region(db);
        Self {
            db,
            hir_ty_menu,
            syn_expr_region_data: syn_expr_region.data(db),
            sema_expr_region_data: sema_expr_region.data(db),
            hir_expr_region,
            hir_expr_source_map,
        }
    }

    pub(crate) fn hir_ty_menu(&self) -> &HirTypeMenu {
        self.hir_ty_menu
    }

    pub(crate) fn return_ty_before_eq(
        &self,
        return_ty_syndicate: Option<ReturnTypeBeforeEqSyndicate>,
    ) -> HirType {
        return_ty_syndicate
            .map(|syndicate| self.hir_ty(syndicate.syn_expr_idx()))
            .unwrap_or(self.hir_ty_menu.unit_ty().into())
    }

    pub(crate) fn return_ty_before_colon(
        &self,
        return_ty_syndicate: Option<ReturnTypeBeforeColonSyndicate>,
    ) -> HirType {
        return_ty_syndicate
            .map(|syndicate| self.hir_ty(syndicate.syn_expr_idx()))
            .unwrap_or(self.hir_ty_menu.unit_ty().into())
    }

    pub(crate) fn hir_ty(&self, syn_expr_idx: SynExprIdx) -> HirType {
        let sema_expr_idx = self
            .sema_expr_region_data
            .syn_root_to_sema_expr_idx(syn_expr_idx);
        match self
            .sema_expr_region_data
            .sema_expr_term(sema_expr_idx)
            .unwrap()
            .unwrap()
            .base_resolved_inner(
                self.sema_expr_region_data
                    .fluffy_term_region()
                    .hollow_terms(),
            ) {
            FluffyTermBase::Ethereal(term) => HirType::from_ethereal(term, self.db),
            FluffyTermBase::Solid(_) => todo!(),
            FluffyTermBase::Hollow(_) => todo!(),
            FluffyTermBase::Place => todo!(),
        }
    }

    pub(crate) fn finish(self) -> HirExprRegion {
        self.hir_expr_region
    }

    pub(crate) fn current_syn_symbol_term(
        &self,
        current_syn_symbol_idx: CurrentSynSymbolIdx,
    ) -> EtherealTerm {
        match self.sema_expr_region_data.symbol_terms()[current_syn_symbol_idx]
            .base_resolved_inner(self.sema_expr_region_data.fluffy_term_region())
        {
            FluffyTermBase::Ethereal(symbol_term) => symbol_term,
            FluffyTermBase::Solid(_) => todo!(),
            FluffyTermBase::Hollow(_) => todo!(),
            FluffyTermBase::Place => todo!(),
        }
    }

    pub(crate) fn db(&self) -> &::salsa::Db {
        self.db
    }

    pub(crate) fn hir_eager_pattern_expr_idx(
        &self,
        syn_pattern_root: impl Into<SynPatternExprRoot>,
    ) -> HirEagerPatternExprIdx {
        let HirExprSourceMap::Eager(source_map) = self.hir_expr_source_map else {
            unreachable!()
        };
        let db = self.db;
        source_map
            .data(db)
            .syn_pattern_root_to_sema_expr_idx(syn_pattern_root)
    }

    pub(crate) fn hir_lazy_pattern_expr_idx(
        &self,
        syn_pattern_root: impl Into<SynPatternExprRoot>,
    ) -> HirLazyPatternExprIdx {
        let HirExprSourceMap::Lazy(source_map) = self.hir_expr_source_map else {
            unreachable!()
        };
        let db = self.db;
        source_map
            .data(db)
            .syn_pattern_root_to_sema_expr_idx(syn_pattern_root)
    }

    pub(crate) fn syn_expr_region_data(&self) -> &'a SynExprRegionData {
        self.syn_expr_region_data
    }
}
