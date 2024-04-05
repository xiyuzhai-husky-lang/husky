pub mod control_flow;

use crate::{source_map::HirLazyExprSourceMap, *};
use husky_entity_path::ItemPath;
use husky_sem_expr::{SemaExprDb, SemaExprRegion};
use husky_syn_defn::item_syn_defn;
use husky_syn_defn::ItemSynDefn;
use husky_syn_expr::SynExprRegion;

#[inline(always)]
pub fn hir_lazy_body_with_expr_region(
    item_path: ItemPath,
    db: &::salsa::Db,
) -> Option<(HirLazyExprIdx, HirLazyExprRegion)> {
    let ItemSynDefn {
        body,
        syn_expr_region,
    } = item_syn_defn(db, item_path)?;
    let sem_expr_region = db.sem_expr_region(syn_expr_region);
    let (hir_lazy_expr_region, hir_lazy_source_map) =
        hir_lazy_expr_region_with_source_map(db, sem_expr_region);
    let hir_lazy_source_map_data = hir_lazy_source_map.data(db);
    let body = sem_expr_region.data(db).syn_root_to_sem_expr_idx(body);
    let Some(body) = hir_lazy_source_map_data.sem_to_hir_lazy_expr_idx(body) else {
        todo!()
    };
    Some((body, hir_lazy_expr_region))
}

pub fn hir_lazy_expr_region_from_syn(
    syn_expr_region: SynExprRegion,
    db: &::salsa::Db,
) -> HirLazyExprRegion {
    let sem_expr_region = db.sem_expr_region(syn_expr_region);
    hir_lazy_expr_region_from_sema(sem_expr_region, db)
}

pub fn hir_lazy_expr_source_map_from_syn(
    syn_expr_region: SynExprRegion,
    db: &::salsa::Db,
) -> HirLazyExprSourceMap {
    let sem_expr_region = db.sem_expr_region(syn_expr_region);
    hir_lazy_expr_region_with_source_map(db, sem_expr_region).1
}

pub fn hir_lazy_expr_region_from_sema(
    sem_expr_region: SemaExprRegion,
    db: &::salsa::Db,
) -> HirLazyExprRegion {
    hir_lazy_expr_region_with_source_map(db, sem_expr_region).0
}
