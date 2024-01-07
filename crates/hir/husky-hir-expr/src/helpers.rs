use husky_entity_kind::{
    FugitiveKind, TraitItemKind,
    TypeItemKind::{self},
};
use husky_entity_path::{region::RegionPath, AssociatedItemPath, ItemPath, MajorItemPath};
use husky_hir_eager_expr::{
    builder::hir_eager_expr_region_with_source_map,
    helpers::{hir_eager_body_with_expr_region, hir_eager_expr_region},
};
use husky_hir_lazy_expr::{
    builder::hir_lazy_expr_region_with_source_map,
    helpers::{hir_lazy_body_with_expr_region, hir_lazy_expr_region_from_syn},
};
use husky_sema_expr::{helpers::analysis::sema_expr_region_contains_gn, SemaExprDb};
use husky_syn_expr::SynExprRegion;

use crate::{source_map::HirExprSourceMap, *};
use husky_syn_defn::{item_syn_defn, ItemSynDefn};

pub fn hir_body_with_expr_region(
    path: ItemPath,
    db: &::salsa::Db,
) -> Option<(HirExprIdx, HirExprRegion)> {
    let ItemSynDefn {
        body: _,
        syn_expr_region,
    } = item_syn_defn(db, path)?;
    let sema_expr_region = db.sema_expr_region(syn_expr_region);
    Some(match is_lazy(sema_expr_region, db) {
        true => {
            let (body, expr_region) = hir_lazy_body_with_expr_region(path, db)?;
            (body.into(), expr_region.into())
        }
        false => {
            let (body, expr_region) = hir_eager_body_with_expr_region(path.into(), db)?;
            (body.into(), expr_region.into())
        }
    })
}

pub fn hir_expr_region(syn_expr_region: SynExprRegion, db: &::salsa::Db) -> HirExprRegion {
    let sema_expr_region = db.sema_expr_region(syn_expr_region);
    match sema_expr_region_contains_gn(db, sema_expr_region) {
        true => hir_lazy_expr_region_from_syn(syn_expr_region, db).into(),
        false => hir_eager_expr_region(syn_expr_region, db).into(),
    }
}

pub fn hir_expr_region_with_source_map(
    syn_expr_region: SynExprRegion,
    db: &::salsa::Db,
) -> (HirExprRegion, HirExprSourceMap) {
    let sema_expr_region = db.sema_expr_region(syn_expr_region);
    let lazy = is_lazy(sema_expr_region, db);
    if lazy {
        let (hir_lazy_expr_region, source_map) =
            hir_lazy_expr_region_with_source_map(db, sema_expr_region);
        (hir_lazy_expr_region.into(), source_map.into())
    } else {
        let (hir_eager_expr_region, source_map) =
            hir_eager_expr_region_with_source_map(db, sema_expr_region);
        (hir_eager_expr_region.into(), source_map.into())
    }
}

fn is_lazy(sema_expr_region: husky_sema_expr::SemaExprRegion, db: &salsa::Db) -> bool {
    match sema_expr_region.path(db).region_path(db).unwrap() {
        RegionPath::Snippet(_) =>
        /* ad hoc */
        {
            false
        }
        RegionPath::Decl(path) | RegionPath::Defn(path) => match path {
            ItemPath::MajorItem(path) => match path {
                MajorItemPath::Fugitive(path) => match path.fugitive_kind(db) {
                    FugitiveKind::FunctionGn => true,
                    FugitiveKind::Val => sema_expr_region_contains_gn(db, sema_expr_region),
                    _ => false,
                },
                _ => false,
            },
            ItemPath::AssociatedItem(path) => match path {
                AssociatedItemPath::TypeItem(path) => match path.item_kind(db) {
                    TypeItemKind::AssociatedVal => {
                        sema_expr_region_contains_gn(db, sema_expr_region)
                    }
                    _ => false,
                },
                AssociatedItemPath::TraitItem(path) => match path.item_kind(db) {
                    TraitItemKind::AssociatedVal => {
                        sema_expr_region_contains_gn(db, sema_expr_region)
                    }
                    _ => false,
                },
                AssociatedItemPath::TraitForTypeItem(path) => match path.item_kind(db) {
                    TraitItemKind::AssociatedVal => {
                        sema_expr_region_contains_gn(db, sema_expr_region)
                    }
                    _ => false,
                },
            },
            _ => false,
        },
    }
}
