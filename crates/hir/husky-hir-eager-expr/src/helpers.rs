use crate::*;
use husky_sema_expr::{SemaExprDb, SemaExprRegion};
use husky_syn_defn::{item_syn_defn, ItemSynDefn};
use husky_syn_expr::{SynExprIdx, SynExprRegion};

pub fn hir_eager_body_with_expr_region(
    item_path: ItemPath,
    db: &::salsa::Db,
) -> Option<(HirEagerExprIdx, HirEagerExprRegion)> {
    let ItemSynDefn {
        body,
        syn_expr_region,
    } = item_syn_defn(db, item_path)?;
    let sema_expr_region = db.sema_expr_region(syn_expr_region);
    let (hir_eager_expr_region, hir_eager_source_map) =
        hir_eager_expr_region_with_source_map(db, sema_expr_region);
    let hir_eager_source_map_data = hir_eager_source_map.data(db);
    let body = sema_expr_region.data(db).syn_root_to_sema_expr_idx(body);
    let Some(body) = hir_eager_source_map_data.sema_to_hir_eager_expr_idx(body) else {
        todo!()
    };
    Some((body, hir_eager_expr_region))
}

pub fn hir_eager_expr_region(
    syn_expr_region: SynExprRegion,
    db: &::salsa::Db,
) -> HirEagerExprRegion {
    let sema_expr_region = db.sema_expr_region(syn_expr_region);
    hir_eager_expr_region_with_source_map(db, sema_expr_region).0
}

pub fn hir_eager_expr_source_map_from_sema(
    sema_expr_region: SemaExprRegion,
    db: &::salsa::Db,
) -> HirEagerExprSourceMap {
    hir_eager_expr_region_with_source_map(db, sema_expr_region).1
}
