use crate::{sem_expr_region, SemExprRegion};
use husky_entity_path::region::RegionPath;
use husky_syn_decl::decl::HasSynDecl;
use husky_syn_defn::item_syn_defn;
use husky_syn_expr::region::SynExprRegion;

pub fn sem_expr_region_from_region_path(
    region_path: RegionPath,
    db: &::salsa::Db,
) -> Option<SemExprRegion> {
    let syn_expr_region = syn_expr_region_from_region_path(region_path, db)?;
    Some(sem_expr_region(db, syn_expr_region))
}

pub fn syn_expr_region_from_region_path(
    region_path: RegionPath,
    db: &::salsa::Db,
) -> Option<SynExprRegion> {
    match region_path {
        RegionPath::CrateDecl(crate_path) => {
            Some(crate_path.syn_decl(db).unwrap()?.syn_expr_region(db))
        }
        RegionPath::Script(_) => todo!(),
        RegionPath::ItemDecl(item_path) => {
            Some(item_path.syn_decl(db).unwrap().syn_expr_region(db)?)
        }
        RegionPath::ItemDefn(item_path) => Some(item_syn_defn(db, item_path)?.syn_expr_region),
    }
}
