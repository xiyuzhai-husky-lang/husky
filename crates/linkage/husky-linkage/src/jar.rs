use crate::*;

#[salsa::jar(db = LinkageDb)]
pub struct LinkageJar(
    crate::linkage::Linkage,
    crate::dependency::linkage_path_dependencies,
    crate::root::crate_specific_linkages,
    crate::version_stamp::LinkageVersionStamp,
    crate::template_argument::ty::LinkageTypePathLeading,
    crate::template_argument::ty::LinkageTypeRitchie,
    crate::pantheon::valkyrie_linkage_pantheon,
    crate::valkyrie::item_valkyrie_rides,
);
