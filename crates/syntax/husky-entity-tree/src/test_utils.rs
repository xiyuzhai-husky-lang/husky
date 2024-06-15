use crate::{
    helpers::paths::module_item_syn_node_paths,
    node::{assoc_item::AssocItemSynNodePath, impl_block::ImplBlockSynNodePath, ItemSynNodePath},
    region_path::SynNodeRegionPath,
};
use husky_entity_path::path::impl_block::TypeSketch;
use husky_path_utils::{Path, PathBuf};
use husky_vfs::{
    path::{crate_path::CratePath, module_path::ModulePath},
    test_utils::*,
};

impl IsVfsTestUnit for ItemSynNodePath {
    fn collect_from_package_path_aux(
        db: &salsa::Db,
        package_path: husky_vfs::path::package_path::PackagePath,
    ) -> impl Iterator<Item = Self> {
        ModulePath::collect_from_package_path_aux(db, package_path)
            .into_iter()
            .map(|module_path| module_item_syn_node_paths(db, module_path))
            .flatten()
            .copied()
    }

    fn determine_expect_file_path(
        self,
        db: &salsa::Db,
        package_expect_files_dir: &std::path::Path,
        config: &VfsTestConfig,
    ) -> std::path::PathBuf {
        let stem = self.stem(package_expect_files_dir, config, db);
        stem.with_extension(config.expect_file_extension().str())
    }

    fn determine_adversarial_path(
        self,
        db: &salsa::Db,
        adversarial_kind: AdversarialKind,
        package_adversarials_dir: &std::path::Path,
        config: &VfsTestConfig,
    ) -> Option<std::path::PathBuf> {
        let stem = self.stem(package_adversarials_dir, config, db);
        Some(stem.with_extension(format!(
            "{}.{}",
            adversarial_kind.as_str(),
            config.adversarial_extension()
        )))
    }

    fn vfs_test_unit_downcast_as_module_path(
        self,
    ) -> Option<husky_vfs::path::module_path::ModulePath> {
        None
    }
}
impl ItemSynNodePath {
    fn stem(self, dir: &Path, config: &VfsTestConfig, db: &::salsa::Db) -> PathBuf {
        let module_path = self.module_path(db);
        let dir = module_path
            .relative_stem(db)
            .to_logical_path(dir.join(config.test_name()));
        let index = module_item_syn_node_paths(db, module_path)
            .iter()
            .position(|&path| path == self)
            .unwrap();
        // ad hoc
        dir.join(format!("{index}-{}", self.stem_inner(db)))
    }

    fn stem_inner(self, db: &::salsa::Db) -> String {
        match self {
            ItemSynNodePath::Submodule(_, slf) => slf.ident(db).data(db).to_string(),
            ItemSynNodePath::MajorItem(slf) => slf.ident(db).data(db).to_string(),
            ItemSynNodePath::TypeVariant(_, slf) => {
                format!(
                    "{}::{}",
                    slf.parent(db).ident(db).data(db),
                    slf.ident(db).data(db)
                )
            }
            ItemSynNodePath::ImplBlock(slf) => match slf {
                ImplBlockSynNodePath::TypeImplBlock(slf) => {
                    format!("impl-{}", slf.ty_path(db).ident(db).data(db))
                }
                ImplBlockSynNodePath::TraitForTypeImplBlock(slf) => format!(
                    "impl-{}-for-{}",
                    slf.trai_path(db).ident(db).data(db),
                    match slf.ty_sketch(db) {
                        TypeSketch::DeriveAny => "_",
                        TypeSketch::Path(ty_path) => ty_path.ident(db).data(db),
                    }
                ),
                ImplBlockSynNodePath::IllFormedImplBlock(_) => todo!(),
            },
            ItemSynNodePath::AssocItem(slf) => match slf {
                AssocItemSynNodePath::TypeItem(slf) => {
                    format!(
                        "{}::{}",
                        slf.ty_path(db).ident(db).data(db),
                        slf.ident(db).data(db)
                    )
                }
                AssocItemSynNodePath::TraitItem(slf) => {
                    format!(
                        "{}::{}",
                        slf.parent_trai_syn_node_path(db).ident(db).data(db),
                        slf.ident(db).data(db)
                    )
                }
                AssocItemSynNodePath::TraitForTypeItem(slf) => {
                    format!(
                        "({}-as-{})::{}",
                        match slf.ty_sketch(db) {
                            TypeSketch::DeriveAny => "_",
                            TypeSketch::Path(ty_path) => ty_path.ident(db).data(db),
                        },
                        slf.trai_path(db).ident(db).data(db),
                        slf.ident(db).data(db)
                    )
                }

                AssocItemSynNodePath::IllFormedItem(_) => todo!(),
            },
            ItemSynNodePath::Attr(_, slf) => {
                format!(
                    "{}::#{}",
                    slf.parent_syn_node_path(db).stem_inner(db),
                    slf.ident(db).data(db)
                )
            }
            ItemSynNodePath::Script(_, _) => todo!(),
        }
    }
}

impl IsVfsTestUnit for SynNodeRegionPath {
    fn collect_from_package_path_aux(
        db: &salsa::Db,
        package_path: husky_vfs::path::package_path::PackagePath,
    ) -> impl Iterator<Item = Self> {
        CratePath::collect_from_package_path_aux(db, package_path)
            .map(|crate_path| SynNodeRegionPath::CrateDecl(crate_path))
            .chain(
                ItemSynNodePath::collect_from_package_path_aux(db, package_path)
                    .into_iter()
                    .map(|syn_node_path| {
                        [
                            SynNodeRegionPath::ItemDecl(syn_node_path),
                            SynNodeRegionPath::ItemDefn(syn_node_path),
                        ]
                    })
                    .flatten(),
            )
    }

    fn determine_expect_file_path(
        self,
        db: &salsa::Db,
        package_expect_files_dir: &husky_path_utils::Path,
        config: &VfsTestConfig,
    ) -> husky_path_utils::PathBuf {
        match self {
            SynNodeRegionPath::CrateDecl(crate_path) => {
                crate_path.determine_expect_file_path(db, package_expect_files_dir, config)
            }
            SynNodeRegionPath::ItemDecl(syn_node_path) => syn_node_path
                .stem(package_expect_files_dir, config, db)
                .with_extension(format!("decl.{}", config.expect_file_extension().str())),
            SynNodeRegionPath::ItemDefn(syn_node_path) => syn_node_path
                .stem(package_expect_files_dir, config, db)
                .with_extension(format!("defn.{}", config.expect_file_extension().str())),
        }
    }

    fn determine_adversarial_path(
        self,
        db: &salsa::Db,
        adversarial_kind: AdversarialKind,
        package_adversarials_dir: &husky_path_utils::Path,
        config: &VfsTestConfig,
    ) -> Option<husky_path_utils::PathBuf> {
        match self {
            SynNodeRegionPath::CrateDecl(crate_path) => crate_path.determine_adversarial_path(
                db,
                adversarial_kind,
                package_adversarials_dir,
                config,
            ),
            SynNodeRegionPath::ItemDecl(syn_node_path) => Some(
                syn_node_path
                    .stem(package_adversarials_dir, config, db)
                    .with_extension(format!(
                        "decl.{}.{}",
                        adversarial_kind.as_str(),
                        config.adversarial_extension(),
                    )),
            ),
            SynNodeRegionPath::ItemDefn(syn_node_path) => Some(
                syn_node_path
                    .stem(package_adversarials_dir, config, db)
                    .with_extension(format!(
                        "defn.{}.{}",
                        adversarial_kind.as_str(),
                        config.adversarial_extension(),
                    )),
            ),
        }
    }

    fn vfs_test_unit_downcast_as_module_path(self) -> Option<ModulePath> {
        None
    }
}
