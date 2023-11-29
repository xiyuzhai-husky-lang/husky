use crate::{instantiation::LinkageInstantiation, path::LinkageItemPath, *};
use husky_entity_path::ItemPathId;
use husky_hir_decl::HasHirDecl;
use husky_hir_defn::HasHirDefn;
use husky_hir_ty::HirTemplateArguments;

#[derive(Debug, PartialEq, Eq)]
pub struct ValkyrieRide {
    linkage_path: LinkageItemPath,
    hir_template_arguments: HirTemplateArguments,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ValkyrieRides {
    rides: Vec<ValkyrieRide>,
    instantiation: LinkageInstantiation,
}

#[salsa::tracked(jar = LinkageJar, return_ref)]
fn item_valkyrie_rides(db: &::salsa::Db, id: ItemPathId) -> Option<ValkyrieRides> {
    let item_path = id.item_path(db);
    let hir_decl_expr_region = item_path.hir_decl(db)?.hir_expr_region(db);
    item_path.hir_defn(db);
    todo!()
}
