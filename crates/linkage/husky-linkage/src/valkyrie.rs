use crate::{instantiation::LinkageInstantiation, path::LinkageItemPath, *};
use husky_entity_path::ItemPathId;
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
fn item_valkyrie_rides(db: &::salsa::Db, path: ItemPathId) -> ValkyrieRides {
    todo!()
}
