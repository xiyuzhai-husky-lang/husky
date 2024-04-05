use crate::{region::SemaPlaceContractRegion, site::SemaPlaceContractSite};
use husky_place::place::EthPlace;
use husky_sem_expr::{SemaExprIdx, SemaExprMap, SemaExprRegion, SemaExprRegionData};

pub struct PlaceContractEngine<'a> {
    db: &'a ::salsa::Db,
    sem_expr_region: SemaExprRegion,
    sem_expr_region_data: &'a SemaExprRegionData,
    expr_sites: SemaExprMap<SemaPlaceContractSite>,
}

impl<'a> PlaceContractEngine<'a> {
    pub fn new(db: &'a ::salsa::Db, sem_expr_region: SemaExprRegion) -> Self {
        let sem_expr_region_data = sem_expr_region.data(db);
        Self {
            db,
            sem_expr_region,
            sem_expr_region_data,
            expr_sites: SemaExprMap::new(sem_expr_region_data.sem_expr_arena()),
        }
    }

    pub(crate) fn db(&self) -> &'a ::salsa::Db {
        self.db
    }

    pub(crate) fn sem_expr_region(&self) -> SemaExprRegion {
        self.sem_expr_region
    }

    pub(crate) fn sem_expr_region_data(&self) -> &'a SemaExprRegionData {
        self.sem_expr_region_data
    }

    pub(crate) fn place(&self, expr: SemaExprIdx) -> Option<EthPlace> {
        match expr.ty(self.sem_expr_region_data.sem_expr_arena2()).quary() {
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
