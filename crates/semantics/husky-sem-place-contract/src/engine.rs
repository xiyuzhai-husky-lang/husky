use crate::{region::SemPlaceContractRegion, site::SemPlaceContractSite};
use husky_place::place::EthPlace;
use husky_sem_expr::{SemExprIdx, SemExprMap, SemExprRegion, SemExprRegionData};

pub struct PlaceContractEngine<'a> {
    db: &'a ::salsa::Db,
    sem_expr_region: SemExprRegion,
    sem_expr_region_data: &'a SemExprRegionData,
    expr_sites: SemExprMap<SemPlaceContractSite>,
}

impl<'a> PlaceContractEngine<'a> {
    pub fn new(db: &'a ::salsa::Db, sem_expr_region: SemExprRegion) -> Self {
        let sem_expr_region_data = sem_expr_region.data(db);
        Self {
            db,
            sem_expr_region,
            sem_expr_region_data,
            expr_sites: SemExprMap::new(sem_expr_region_data.sem_expr_arena()),
        }
    }

    pub(crate) fn db(&self) -> &'a ::salsa::Db {
        self.db
    }

    pub(crate) fn sem_expr_region(&self) -> SemExprRegion {
        self.sem_expr_region
    }

    pub(crate) fn sem_expr_region_data(&self) -> &'a SemExprRegionData {
        self.sem_expr_region_data
    }

    pub(crate) fn place(&self, expr: SemExprIdx) -> Option<EthPlace> {
        match expr.ty(self.sem_expr_region_data.sem_expr_arena2()).quary() {
            Some(quary) => quary.place(),
            None => None,
        }
    }
}

/// # actions
impl<'a> PlaceContractEngine<'a> {
    pub(crate) fn set_expr_site(&mut self, expr: SemExprIdx, site: SemPlaceContractSite) {
        self.expr_sites.insert_new(expr, site)
    }

    pub(crate) fn finish(self) -> SemPlaceContractRegion {
        SemPlaceContractRegion::new(self.expr_sites)
    }
}
