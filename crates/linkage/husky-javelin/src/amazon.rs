use super::*;
use crate::{instantiation::JavInstantiation, javelin::JavelinData, path::JavPath};
use husky_entity_path::path::ItemPath;
use husky_entity_tree::helpers::paths::{crate_module_paths, module_item_paths};
use husky_hir_decl::parameter::template::item_hir_template_parameter_stats;
use husky_vfs::path::package_path::PackagePath;
use vec_like::VecSet;

/// an Amazon javelin is one with univalent instantiation
#[salsa::as_id]
#[salsa::deref_id]
#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AmazonJavelin(Javelin);

impl AmazonJavelin {
    pub fn from_item_path(path: ItemPath, db: &::salsa::Db) -> Option<Self> {
        let stats = item_hir_template_parameter_stats(db, *path)?;
        if stats.tys + stats.constants > 0 {
            return None;
        }
        Some(AmazonJavelin(Javelin::new(
            db,
            JavelinData::PathLeading {
                path: JavPath::try_from_item_path(path, db)?,
                // ad hoc consider places
                instantiation: JavInstantiation::new_amazon(path),
            },
        )))
    }
}

#[salsa::tracked(return_ref)]
pub(crate) fn package_amazon_javelins(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> VecSet<AmazonJavelin> {
    let mut amazon_javelins: VecSet<AmazonJavelin> = Default::default();
    for &crate_path in package_path
        .crate_paths(db)
        .expect("no vfs error at this stage")
    {
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
    DB::ast_expect_test_debug_with_db(
        package_amazon_javelins,
        &AstTestConfig::new(
            "package_amazon_javelins",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::LINKAGE,
        ),
    )
}
