use crate::{sem_expr_region, SemExprRegion};
use husky_entity_path::region::RegionPath;
use husky_syn_decl::decl::HasSynDecl;
use husky_syn_defn::item_syn_defn;

pub fn sem_expr_region_from_region_path(
    region_path: RegionPath,
    db: &::salsa::Db,
) -> SemExprRegion {
    let syn_expr_region = match region_path {
        RegionPath::CrateDecl(_) => todo!(),
        RegionPath::Script(_) => todo!(),
        RegionPath::ItemDecl(item_path) => {
            item_path.syn_decl(db).unwrap().syn_expr_region(db).unwrap()
        }
        RegionPath::ItemDefn(item_path) => item_syn_defn(db, item_path).unwrap().syn_expr_region,
    };
    sem_expr_region(db, syn_expr_region)
}
