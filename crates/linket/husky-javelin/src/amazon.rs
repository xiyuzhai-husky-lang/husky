use super::*;
use crate::{instantiation::JavInstantiation, javelin::JavelinData, path::JavPath};
use husky_entity_kind::MajorFormKind;
use husky_entity_path::path::{major_item::MajorItemPath, ItemPath};
use husky_entity_tree::helpers::paths::{crate_module_paths, module_item_paths};
use husky_hir_decl::parameter::template::item_hir_template_parameter_stats;
use husky_sem_var_deps::{item_sem_var_deps, var_deps::SemVarDep};
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
        for dep in item_sem_var_deps(path, db) {
            match dep {
                SemVarDep::Item(dep_item_path) => match dep_item_path {
                    ItemPath::Submodule(_, _) => todo!(),
                    ItemPath::MajorItem(dep_major_item_path) => match dep_major_item_path {
                        MajorItemPath::Type(_) => todo!(),
                        MajorItemPath::Trait(_) => todo!(),
                        MajorItemPath::Form(dep_major_form_path) => {
                            match dep_major_form_path.kind(db) {
                                MajorFormKind::Ritchie(_) => todo!(),
                                MajorFormKind::TypeAlias => todo!(),
                                MajorFormKind::TypeVar => {
                                    // amazon javelin shouldn't have any type var dep
                                    return None;
                                }
                                MajorFormKind::Val => (),
                                MajorFormKind::StaticMut => (),
                                MajorFormKind::StaticVar => (),
                                MajorFormKind::Compterm => todo!(),
                                MajorFormKind::Conceptual => todo!(),
                            }
                        }
                    },
                    ItemPath::AssocItem(_) => todo!(),
                    ItemPath::TypeVariant(_, _) => todo!(),
                    ItemPath::ImplBlock(_) => todo!(),
                    ItemPath::Attr(_, _) => todo!(),
                    ItemPath::Script(_, _) => todo!(),
                },
            }
        }
        let stats = item_hir_template_parameter_stats(db, *path)?;
        if stats.self_ty + stats.tys + stats.constants > 0 {
            return None;
        }
        let path = JavPath::try_from_item_path(path, db)?;
        Some(AmazonJavelin(Javelin::new(
            db,
            JavelinData::PathLeading {
                path,
                // ad hoc consider places
                instantiation: JavInstantiation::new_amazon(path.into(), db),
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
    DB::ast_rich_test_debug_with_db(
        package_amazon_javelins,
        &AstTestConfig::new(
            "package_amazon_javelins",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::LINKET,
        ),
    )
}
