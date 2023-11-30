use super::*;
use crate::{
    instantiation::JavelinInstantiation, jar::JavelinJar, javelin::JavelinData, path::JavelinPath,
};
use husky_entity_path::ItemPath;
use husky_entity_syn_tree::helpers::paths::{crate_module_paths, module_item_paths};
use husky_hir_decl::parameter::template::item_hir_template_parameter_stats;
use husky_vfs::{CratePath, PackagePath};
use vec_like::VecSet;

/// an Amazon javelin is one with empty instantiation
#[salsa::as_id]
#[salsa::deref_id]
#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AmazonJavelin(Javelin);

impl AmazonJavelin {
    pub fn from_item_path(item_path: ItemPath, db: &::salsa::Db) -> Option<Self> {
        let stats = item_hir_template_parameter_stats(db, *item_path)?;
        if stats.tys + stats.constants > 0 {
            return None;
        }
        Some(AmazonJavelin(Javelin::new(
            db,
            JavelinData::PathLeading {
                path: JavelinPath::try_from_item_path(item_path)?,
                // ad hoc consider places
                instantiation: JavelinInstantiation::new_first_born(),
            },
        )))
    }
}

#[salsa::tracked(jar = JavelinJar, return_ref)]
pub(crate) fn package_amazon_javelins(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> VecSet<AmazonJavelin> {
    let mut amazon_javelins: VecSet<AmazonJavelin> = Default::default();
    for &crate_path in package_path.crate_paths(db) {
        for &module_path in crate_module_paths(db, crate_path) {
            for &item_path in module_item_paths(db, module_path) {
                if let Some(amazon_javelin) = AmazonJavelin::from_item_path(item_path, db) {
                    amazon_javelins.insert(amazon_javelin)
                }
                // todo: type variants etc
            }
        }
    }
    amazon_javelins
}

#[test]
fn package_amazon_javelins_works() {
    let mut db = DB::default();
    db.ast_expect_test_debug_with_db(
        package_amazon_javelins,
        &AstTestConfig::new("package_amazon_javelins"),
    )
}
