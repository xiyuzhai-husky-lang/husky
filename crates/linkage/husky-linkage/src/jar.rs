use crate::*;

#[salsa::jar(db = LinkageDb)]
pub struct LinkageJar(
    crate::linkage::Linkage,
    crate::linkage::linkages_emancipated_by_javelin,
    crate::linkage::package_linkages,
    crate::template_argument::ty::LinkageTypePathLeading,
    crate::template_argument::ty::LinkageTypeRitchie,
);
