use husky_entity_kind::{MajorFormKind, TraitItemKind, TypeItemKind};
use husky_entity_path::{
    path::{assoc_item::AssocItemPath, major_item::MajorItemPath, ItemPath},
    region::RegionPath,
};
use husky_hir_eager_expr::{
    builder::hir_eager_expr_region_with_source_map,
    helpers::region::{hir_eager_body_with_expr_region, hir_eager_expr_region},
};
use husky_hir_lazy_expr::{
    builder::hir_lazy_expr_region_with_source_map,
    helpers::{hir_lazy_body_with_expr_region, hir_lazy_expr_region_from_syn},
};
use husky_sem_expr::{helpers::analysis::sem_expr_region_requires_lazy, SemExprDb};
use husky_syn_expr::region::SynExprRegion;

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
    let sem_expr_region = db.sem_expr_region(syn_expr_region);
    Some(match is_lazy(sem_expr_region, db) {
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
    let sem_expr_region = db.sem_expr_region(syn_expr_region);
    match sem_expr_region_requires_lazy(db, sem_expr_region) {
        true => hir_lazy_expr_region_from_syn(syn_expr_region, db).into(),
        false => hir_eager_expr_region(syn_expr_region, db).into(),
    }
}

pub fn hir_expr_region_with_source_map(
    syn_expr_region: SynExprRegion,
    db: &::salsa::Db,
) -> (HirExprRegion, HirExprSourceMap) {
    let sem_expr_region = db.sem_expr_region(syn_expr_region);
    let lazy = is_lazy(sem_expr_region, db);
    if lazy {
        let (hir_lazy_expr_region, source_map) =
            hir_lazy_expr_region_with_source_map(db, sem_expr_region);
        (hir_lazy_expr_region.into(), source_map.into())
    } else {
        let (hir_eager_expr_region, source_map) =
            hir_eager_expr_region_with_source_map(db, sem_expr_region);
        (hir_eager_expr_region.into(), source_map.into())
    }
}

fn is_lazy(sem_expr_region: husky_sem_expr::SemExprRegion, db: &salsa::Db) -> bool {
    match sem_expr_region.path(db) {
        RegionPath::CrateDecl(_) => todo!(),
        RegionPath::Script(_) =>
        /* ad hoc */
        {
            false
        }
        RegionPath::ItemDecl(path) | RegionPath::ItemDefn(path) => match path {
            ItemPath::MajorItem(path) => match path {
                MajorItemPath::Form(path) => match path.major_form_kind(db) {
                    MajorFormKind::GN | MajorFormKind::QN => true,
                    MajorFormKind::Val | MajorFormKind::VN | MajorFormKind::TN => {
                        sem_expr_region_requires_lazy(db, sem_expr_region)
                    }
                    _ => false,
                },
                _ => false,
            },
            ItemPath::AssocItem(path) => match path {
                AssocItemPath::TypeItem(path) => match path.item_kind(db) {
                    TypeItemKind::AssocRitchie(ritchie_item_kind)
                    | TypeItemKind::MethodRitchie(ritchie_item_kind) => ritchie_item_kind.is_lazy(),
                    TypeItemKind::AssocVal => sem_expr_region_requires_lazy(db, sem_expr_region),
                    _ => false,
                },
                AssocItemPath::TraitItem(path) => match path.item_kind(db) {
                    TraitItemKind::AssocRitchie(ritchie_item_kind)
                    | TraitItemKind::MethodRitchie(ritchie_item_kind) => {
                        ritchie_item_kind.is_lazy()
                    }
                    TraitItemKind::AssocVal => sem_expr_region_requires_lazy(db, sem_expr_region),
                    _ => false,
                },
                AssocItemPath::TraitForTypeItem(path) => match path.item_kind(db) {
                    TraitItemKind::AssocRitchie(ritchie_item_kind)
                    | TraitItemKind::MethodRitchie(ritchie_item_kind) => {
                        ritchie_item_kind.is_lazy()
                    }
                    TraitItemKind::AssocVal => sem_expr_region_requires_lazy(db, sem_expr_region),
                    _ => false,
                },
            },
            _ => false,
        },
    }
}
