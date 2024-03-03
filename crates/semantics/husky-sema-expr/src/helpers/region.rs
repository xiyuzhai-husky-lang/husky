use crate::{sema_expr_region, SemaExprRegion};
use husky_entity_path::region::RegionPath;
use husky_syn_decl::decl::HasSynDecl;
use husky_syn_defn::item_syn_defn;

pub fn sema_expr_region_from_region_path(
    region_path: RegionPath,
    db: &::salsa::Db,
) -> SemaExprRegion {
    let syn_expr_region = match region_path {
        RegionPath::Snippet(_) => todo!(),
        RegionPath::Decl(item_path) => item_path.syn_decl(db).unwrap().syn_expr_region(db).unwrap(),
        RegionPath::Defn(item_path) => item_syn_defn(db, item_path).unwrap().syn_expr_region,
    };
    sema_expr_region(db, syn_expr_region)
}
