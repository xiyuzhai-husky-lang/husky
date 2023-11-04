use crate::db::HirDeclDb;
use husky_hir_expr::HirExprRegion;
use husky_hir_ty::{menu::HirTypeMenu, HirType};
use husky_syn_expr::{ReturnTypeBeforeEqSyndicate, SynExprIdx, SynExprRegion};

pub(crate) struct HirDeclBuilder<'a> {
    db: &'a dyn HirDeclDb,
    hir_ty_menu: &'a HirTypeMenu,
    hir_expr_region: HirExprRegion,
}

impl<'a> HirDeclBuilder<'a> {
    pub(crate) fn new(syn_expr_region: SynExprRegion, db: &'a dyn HirDeclDb) -> Self {
        todo!()
    }

    pub(crate) fn hir_ty_menu(&self) -> &HirTypeMenu {
        self.hir_ty_menu
    }

    pub(crate) fn return_ty(&self, syn_expr_idx: Option<ReturnTypeBeforeEqSyndicate>) -> HirType {
        todo!()
    }

    pub(crate) fn hir_ty(&self, syn_expr_idx: SynExprIdx) -> HirType {
        todo!()
    }

    pub(crate) fn finish(self) -> HirExprRegion {
        todo!()
    }
}
