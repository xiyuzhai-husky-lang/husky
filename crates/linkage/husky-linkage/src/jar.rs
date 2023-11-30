use crate::*;

#[salsa::jar(db = LinkageDb)]
pub struct LinkageJar(
    crate::linkage::Linkage,
    crate::dependency::linkage_path_dependencies,
    crate::version_stamp::LinkageVersionStamp,
    crate::template_argument::ty::LinkageTypePathLeading,
    crate::template_argument::ty::LinkageTypeRitchie,
    crate::javelin::amazon::package_amazon_javelins,
    crate::javelin::valkyrie::package_valkyrie_linkage_pantheon,
    crate::javelin::valkyrie::item_valkyrie_rides,
    crate::javelin::valkyrie::javelin_valkyrie_javelins,
);
