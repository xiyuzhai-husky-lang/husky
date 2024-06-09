use super::*;
use husky_entity_path::path::major_item::{connection::MajorItemConnection, form::MajorFormPath};
use husky_entity_tree::helpers::paths::module_test_paths;
use husky_hir_decl::decl::HasHirDecl;
use husky_vfs::{jar::VfsDb, test_utils::*};

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TestLinkage {
    path: MajorFormPath,
    linkage: Linkage,
}

impl std::ops::Deref for TestLinkage {
    type Target = Linkage;

    fn deref(&self) -> &Self::Target {
        &self.linkage
    }
}

impl TestLinkage {
    pub fn path(self, db: &salsa::Db) -> MajorFormPath {
        self.path
    }

    pub fn linkage(self, db: &::salsa::Db) -> Linkage {
        self.linkage
    }
}

impl IsVfsTestUnit for TestLinkage {
    fn collect_from_package_path(
        db: &salsa::Db,
        package_path: husky_vfs::path::package_path::PackagePath,
    ) -> impl Iterator<Item = Self> {
        db.collect_probable_modules(package_path)
            .into_iter()
            .flat_map(|module_path| module_test_paths(db, module_path).iter().copied())
            .map(|path| {
                assert!(path
                    .hir_decl(db)
                    .unwrap()
                    .template_parameters(db)
                    .unwrap()
                    .is_empty());
                TestLinkage {
                    path,
                    linkage: Linkage::new(
                        db,
                        LinkageData::MajorFunctionRitchie {
                            path,
                            instantiation: LinInstantiation::new_empty(false),
                        },
                    ),
                }
            })
    }

    fn determine_expect_file_path(
        self,
        db: &salsa::Db,
        package_expect_files_dir: &std::path::Path,
        config: &VfsTestConfig,
    ) -> std::path::PathBuf {
        let path = self.path;
        let stem = path
            .module_path(db)
            .relative_stem(db)
            .to_logical_path(package_expect_files_dir);
        match path.data(db).connection() {
            MajorItemConnection::Connected => stem.join(format!(
                "{}.{}",
                path.ident(db).data(db),
                config.expect_file_extension().str()
            )),
            MajorItemConnection::Disconnected(_) => todo!(),
        }
    }

    fn determine_adversarial_path(
        self,
        db: &salsa::Db,
        adversarial_kind: AdversarialKind,
        package_adversarials_dir: &std::path::Path,
        config: &VfsTestConfig,
    ) -> Option<std::path::PathBuf> {
        None
    }

    fn vfs_test_unit_downcast_as_module_path(
        self,
    ) -> Option<husky_vfs::path::module_path::ModulePath> {
        todo!()
    }
}
