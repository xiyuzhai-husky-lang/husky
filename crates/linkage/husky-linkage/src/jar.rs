use crate::*;

#[salsa::jar(db = LinkageDb)]
pub struct LinkageJar(
    crate::linkage::Linkage,
    crate::linkage::linkages_emancipated_by_javelin,
    crate::linkage::package_linkages,
    crate::template_argument::ty::LinkageTypePathLeading,
    crate::template_argument::ty::LinkageTypeRitchie,
    crate::version_stamp::LinkageVersionStamp,
    crate::version_stamp::linkage_version_stamp,
    crate::version_stamp::linkage_ty_path_leading_version_stamp,
    crate::version_stamp::linkage_ty_ritchie_version_stamp,
    crate::trai::LinkageTrait,
);
