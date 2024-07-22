pub mod db;

use self::db::DevComptimeDb;

use husky_devsoul::{devsoul::IsDevsoul, linktime::IsLinktime};
use husky_devsoul_interface::HuskyIngredientIndex;
use husky_devsoul_interface::HuskyJarIndex;
use husky_entity_kind::{MajorFormKind, TraitItemKind, TypeItemKind};
use husky_entity_path::path::{assoc_item::AssocItemPath, major_item::MajorItemPath, ItemPath};
use husky_entity_tree::helpers::ingredient::{HasIngredientPaths, IngredientPath};
use husky_ki::Ki;
use husky_ki_repr::{
    repr::{KiCachingClass, KiRepr},
    var_deps::KiStaticVarDeps,
};
use husky_linket::linket::Linket;
use husky_manifest::helpers::upstream::HasAllUpstreamPackages;
use husky_toolchain_config::toolchain_config;
use husky_vfs::{
    error::VfsResult,
    path::{
        crate_path::{CrateKind, CratePath},
        linktime_target_path::LinktimeTargetPath,
        package_path::PackagePath,
    },
};
use std::path::Path;

pub struct DevComptime<Devsoul: IsDevsoul> {
    db: DevComptimeDb,
    target: DevComptimeTarget,
    target_path: Option<LinktimeTargetPath>,
    linktime: Devsoul::Linktime,
    /// first index by jar index,
    /// second index by ingredient index
    ingredient_ki_infos: Vec<(PackagePath, Vec<(IngredientPath, Option<IngredientKiInfo>)>)>,
}

pub struct IngredientKiInfo {
    ki_repr: KiRepr,
    ki: Ki,
    caching_class: KiCachingClass,
    ki_var_deps: KiStaticVarDeps,
}

impl IngredientKiInfo {
    pub fn ki_repr(&self) -> KiRepr {
        self.ki_repr
    }

    pub fn ki(&self) -> Ki {
        self.ki
    }

    pub fn caching_class(&self) -> KiCachingClass {
        self.caching_class
    }

    pub fn ki_var_deps(&self) -> &KiStaticVarDeps {
        &self.ki_var_deps
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DevComptimeTarget {
    None,
    SingleCrate(CratePath),
}

impl<Devsoul: IsDevsoul> DevComptime<Devsoul> {
    pub fn new(target_crate_path: impl AsRef<Path>) -> VfsResult<Self> {
        let target_crate_path = target_crate_path.as_ref();
        let db = DevComptimeDb::default();
        let toolchain = toolchain_config(target_crate_path, &*db).toolchain();
        let target_package_path =
            match PackagePath::new_local_or_toolchain_package(&db, toolchain, target_crate_path) {
                Ok(package_path) => package_path,
                Err(_e) => todo!(),
            };
        let Some(target_crate_path) =
            CratePath::new(target_package_path, CrateKind::Main, &db).into_result_option()?
        else {
            todo!()
        };
        let target = DevComptimeTarget::SingleCrate(target_crate_path);
        let target_path = match target {
            DevComptimeTarget::None => None,
            DevComptimeTarget::SingleCrate(crate_path) => Some(LinktimeTargetPath::new_package(
                crate_path.package_path(&db),
                &db,
            )),
        };
        let ingredient_ki_infos = target_path
            .map(|target_path| ingredient_ki_infos(target_path, &db))
            .unwrap_or_default();
        Ok(Self {
            linktime: IsLinktime::new_linktime(
                /* ad hoc */
                LinktimeTargetPath::new_package(target_crate_path.package_path(&db), &db),
                &db,
            ),
            target,
            target_path,
            db,
            ingredient_ki_infos,
        })
    }

    pub fn target(&self) -> DevComptimeTarget {
        self.target
    }

    pub fn linktime_target_path(&self) -> Option<LinktimeTargetPath> {
        self.target_path
    }

    pub fn linket_impl(&self, linket: Linket) -> Devsoul::LinketImpl {
        self.linktime.linket_impl(linket, self.db())
    }

    #[deprecated]
    pub fn ingredient_ki_and_var_deps(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
    ) -> (Ki, &KiStaticVarDeps) {
        let ingredient_ki_info = &self.ingredient_ki_infos[jar_index.index()].1
            [ingredient_index.index()]
        .1
        .as_ref()
        .unwrap();
        (ingredient_ki_info.ki(), ingredient_ki_info.ki_var_deps())
    }

    pub fn ingredient_ki_repr(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
    ) -> KiRepr {
        self.ingredient_ki_infos[jar_index.index()].1[ingredient_index.index()]
            .1
            .as_ref()
            .unwrap()
            .ki_repr()
    }
}

fn ingredient_ki_infos(
    target_path: LinktimeTargetPath,
    db: &::salsa::Db,
) -> Vec<(PackagePath, Vec<(IngredientPath, Option<IngredientKiInfo>)>)> {
    target_path
        .all_upstream_packages(db)
        .unwrap()
        .iter()
        .map(|&package_path| {
            let crate_path = package_path
                .lib_crate_path(db)
                .or(package_path.main_crate_path(db))
                .unwrap();
            (
                package_path,
                crate_path
                    .ingredient_paths(db)
                    .iter()
                    .map(|&ingredient_path| {
                        let Some(ki_repr) = (match ingredient_path.item_path() {
                            // todo: consider StaticVar, StaticMut?
                            ItemPath::MajorItem(MajorItemPath::Form(path))
                                if path.kind(db) == MajorFormKind::Val =>
                            {
                                Some(KiRepr::new_val(path, db))
                            }
                            ItemPath::AssocItem(path) => match path {
                                AssocItemPath::TypeItem(path) => match path.item_kind(db) {
                                    TypeItemKind::AssocVal => todo!(),
                                    _ => None,
                                },
                                AssocItemPath::TraitItem(path) => match path.item_kind(db) {
                                    TraitItemKind::AssocVal => todo!(),
                                    _ => None,
                                },
                                AssocItemPath::TraitForTypeItem(path) => match path.item_kind(db) {
                                    TraitItemKind::AssocVal => todo!(),
                                    _ => None,
                                },
                            },
                            _ => None,
                        }) else {
                            return (ingredient_path, None);
                        };
                        let ki = ki_repr.ki(db);
                        let caching_class = ki_repr.caching_class(db);
                        let ki_var_deps = ki_repr.var_deps(db).clone();
                        let info = IngredientKiInfo {
                            ki_repr,
                            ki,
                            caching_class,
                            ki_var_deps,
                        };
                        (ingredient_path, Some(info))
                    })
                    .collect(),
            )
        })
        .collect()
}

impl<Devsoul: IsDevsoul> DevComptime<Devsoul> {
    pub fn db(&self) -> &::salsa::Db {
        &self.db
    }
}

impl<Devsoul: IsDevsoul> Default for DevComptime<Devsoul>
where
    Devsoul::Linktime: Default,
{
    fn default() -> Self {
        Self {
            target: DevComptimeTarget::None,
            db: Default::default(),
            linktime: Default::default(),
            target_path: None,
            ingredient_ki_infos: vec![],
        }
    }
}
