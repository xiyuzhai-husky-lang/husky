use crate::{
    error::{SemaPlaceContractError, SemaPlaceContractResult},
    region::SemaPlaceContractRegion,
    site::SemaPlaceContractSite,
};
use husky_place::place::Place;
use husky_sema_expr::{
    SemaExprIdx, SemaExprMap, SemaExprRegion, SemaExprRegionData, SemaStmtIdx, SemaStmtMap,
};

pub struct PlaceContractEngine<'a> {
    db: &'a ::salsa::Db,
    sema_expr_region: SemaExprRegion,
    sema_expr_region_data: &'a SemaExprRegionData,
    expr_sites: SemaExprMap<SemaPlaceContractSite>,
}

impl<'a> PlaceContractEngine<'a> {
    pub fn new(db: &'a ::salsa::Db, sema_expr_region: SemaExprRegion) -> Self {
        let sema_expr_region_data = sema_expr_region.data(db);
        Self {
            db,
            sema_expr_region,
            sema_expr_region_data,
            expr_sites: SemaExprMap::new(sema_expr_region_data.sema_expr_arena()),
        }
    }

    pub(crate) fn db(&self) -> &'a ::salsa::Db {
        self.db
    }

    pub(crate) fn sema_expr_region(&self) -> SemaExprRegion {
        self.sema_expr_region
    }

    pub(crate) fn sema_expr_region_data(&self) -> &'a SemaExprRegionData {
        self.sema_expr_region_data
    }

    pub(crate) fn place(&self, expr: SemaExprIdx) -> Option<Place> {
        match expr
            .ty(self.sema_expr_region_data.sema_expr_arena2())
            .quary()
        {
            Some(quary) => quary.place(),
            None => None,
        }
    }
}

/// # actions
impl<'a> PlaceContractEngine<'a> {
    pub(crate) fn set_expr_site(&mut self, expr: SemaExprIdx, site: SemaPlaceContractSite) {
        self.expr_sites.insert_new(expr, site)
    }

    pub(crate) fn finish(self) -> SemaPlaceContractRegion {
        SemaPlaceContractRegion::new(self.expr_sites)
    }
}
