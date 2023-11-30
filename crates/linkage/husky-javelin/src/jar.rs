use crate::*;

#[salsa::jar(db = JavelinDb)]
pub struct JavelinJar(
    crate::javelin::Javelin,
    crate::version_stamp::JavelinVersionStamp,
    crate::template_argument::ty::JavelinTypePathLeading,
    crate::template_argument::ty::JavelinTypeRitchie,
    crate::javelin::amazon::package_amazon_javelins,
    crate::javelin::valkyrie::package_valkyrie_javelin_pantheon,
    crate::javelin::valkyrie::item_valkyrie_rides,
    crate::javelin::valkyrie::javelin_valkyrie_javelins,
);
