use husky_hir_eager_expr::{
    builder::hir_eager_expr_region_with_source_map,
    helpers::{hir_eager_body_with_expr_region, hir_eager_expr_region},
};
use husky_hir_lazy_expr::{
    builder::hir_lazy_expr_region_with_source_map,
    helpers::{hir_lazy_body_with_expr_region, hir_lazy_expr_region},
};
use husky_sema_expr::helpers::analysis::sema_expr_region_contains_gn;
use husky_syn_expr::SynExprRegion;

use crate::{source_map::HirExprSourceMap, *};

pub fn hir_body_with_expr_region(
    body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
    db: &dyn HirExprDb,
) -> Option<(HirExprIdx, HirExprRegion)> {
    let (body, syn_expr_region) = body_with_syn_expr_region?;
    let sema_expr_region = db.sema_expr_region(syn_expr_region);
    Some(match sema_expr_region_contains_gn(db, sema_expr_region) {
        true => {
            let (body, expr_region) =
                hir_lazy_body_with_expr_region(Some((body, syn_expr_region)), db)?;
            (body.into(), expr_region.into())
        }
        false => {
            let (body, expr_region) =
                hir_eager_body_with_expr_region(Some((body, syn_expr_region)), db)?;
            (body.into(), expr_region.into())
        }
    })
}

pub fn hir_expr_region(syn_expr_region: SynExprRegion, db: &dyn HirExprDb) -> HirExprRegion {
    let sema_expr_region = db.sema_expr_region(syn_expr_region);
    match sema_expr_region_contains_gn(db, sema_expr_region) {
        true => hir_lazy_expr_region(syn_expr_region, db).into(),
        false => hir_eager_expr_region(syn_expr_region, db).into(),
    }
}

pub fn hir_expr_region_with_source_map(
    syn_expr_region: SynExprRegion,
    db: &dyn HirExprDb,
) -> (HirExprRegion, HirExprSourceMap) {
    let sema_expr_region = db.sema_expr_region(syn_expr_region);
    match sema_expr_region_contains_gn(db, sema_expr_region) {
        true => {
            let (hir_lazy_expr_region, source_map) =
                hir_lazy_expr_region_with_source_map(db, sema_expr_region);
            (hir_lazy_expr_region.into(), source_map.into())
        }
        false => {
            let (hir_eager_expr_region, source_map) =
                hir_eager_expr_region_with_source_map(db, sema_expr_region);
            (hir_eager_expr_region.into(), source_map.into())
        }
    }
}
