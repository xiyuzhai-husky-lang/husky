use crate::db::HirDeclDb;
use husky_fluffy_term::FluffyTermBase;
use husky_hir_expr::{
    helpers::hir_expr_region_with_source_map, source_map::HirExprSourceMap, HirExprRegion,
};
use husky_hir_lazy_expr::builder::hir_lazy_expr_region_with_source_map;
use husky_hir_ty::{menu::HirTypeMenu, HirType};
use husky_sema_expr::SemaExprRegionData;
use husky_syn_expr::{ReturnTypeBeforeEqSyndicate, SynExprIdx, SynExprRegion};

pub(crate) struct HirDeclBuilder<'a> {
    db: &'a dyn HirDeclDb,
    hir_ty_menu: &'a HirTypeMenu,
    sema_expr_region_data: &'a SemaExprRegionData,
    hir_expr_region: HirExprRegion,
    hir_expr_source_map: HirExprSourceMap,
}

impl<'a> HirDeclBuilder<'a> {
    pub(crate) fn new(syn_expr_region: SynExprRegion, db: &'a dyn HirDeclDb) -> Self {
        let toolchain = syn_expr_region.data(db).path().toolchain(db);
        let hir_ty_menu = db.hir_ty_menu(toolchain);
        let (hir_expr_region, hir_expr_source_map) =
            hir_expr_region_with_source_map(syn_expr_region, db);
        let sema_expr_region = db.sema_expr_region(syn_expr_region);
        Self {
            db,
            hir_ty_menu,
            sema_expr_region_data: sema_expr_region.data(db),
            hir_expr_region,
            hir_expr_source_map,
        }
    }

    pub(crate) fn hir_ty_menu(&self) -> &HirTypeMenu {
        self.hir_ty_menu
    }

    pub(crate) fn return_ty(
        &self,
        return_ty_syndicate: Option<ReturnTypeBeforeEqSyndicate>,
    ) -> HirType {
        return_ty_syndicate
            .map(|syndicate| self.hir_ty(syndicate.expr()))
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
}
