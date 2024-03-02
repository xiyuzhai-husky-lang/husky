use crate::{engine::PlaceContractEngine, site::SemaPlaceContractSite};
use husky_sema_expr::{SemaExprIdx, SemaExprMap, SemaExprRegion, SemaStmtIdx, SemaStmtMap};

#[derive(Debug, PartialEq, Eq)]
pub struct SemaPlaceContractRegion {
    expr_sites: SemaExprMap<SemaPlaceContractSite>,
}

/// # constructor
impl SemaPlaceContractRegion {
    pub(crate) fn new(expr_sites: SemaExprMap<SemaPlaceContractSite>) -> Self {
        Self { expr_sites }
    }
}

impl std::ops::Index<SemaExprIdx> for SemaPlaceContractRegion {
    type Output = SemaPlaceContractSite;

    fn index(&self, expr: SemaExprIdx) -> &Self::Output {
        &self.expr_sites[expr]
    }
}

#[salsa::tracked(return_ref)]
pub fn sema_place_contract_region(
    db: &::salsa::Db,
    sema_expr_region: SemaExprRegion,
) -> SemaPlaceContractRegion {
    let mut engine = PlaceContractEngine::new(db, sema_expr_region);
    engine.infer_all_exprs();
    engine.finish()
}
