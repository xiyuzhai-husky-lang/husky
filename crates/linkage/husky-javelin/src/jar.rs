#[salsa::jar]
pub struct JavelinJar(
    crate::javelin::Javelin,
    crate::template_argument::ty::JavTypePathLeading,
    crate::template_argument::ty::JavRitchieType,
    crate::amazon::package_amazon_javelins,
    crate::valkyrie::package_valkyrie_javelin_pantheon,
    crate::valkyrie::item_valkyrie_rides,
    crate::valkyrie::javelin_generated_valkyrie_javelins,
);
