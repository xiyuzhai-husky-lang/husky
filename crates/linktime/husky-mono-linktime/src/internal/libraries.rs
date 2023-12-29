use super::*;
use husky_cargo_utils::compile_workspace;
use husky_corgi_config::transpilation_setup::HasTranspilationSetup;
use husky_linkage_impl::AnyLinkageImpls;
use husky_manifest::{has_manifest::HasPackageManifest, HasAllPackages};
use husky_rust_transpilation::{db::RustTranspilationJar, transpile_to_fs::TranspileToFsFull};
use husky_task::IsTask;
use husky_task_interface::TaskJarIndex;
use husky_vfs::{linktime_target_path::LinktimeTargetPathData, PackagePath};
use libloading::Library;
use std::path::PathBuf;
use vec_like::{VecMap, VecPairMap, VecSet};

pub struct MonoLinkageLibraries {
    pub cdylibs: VecPairMap<PackagePath, Cdylib>,
}

#[salsa::debug_with_db]
pub struct Cdylib(TaskJarIndex, Library);

impl Cdylib {
    pub(crate) fn linkage_impls<LinkageImpl: IsLinkageImpl>(&self) -> Vec<LinkageImpl> {
        let package_linkage_impls: libloading::Symbol<fn(TaskJarIndex) -> AnyLinkageImpls> =
            unsafe { self.1.get(b"linkage_impls").unwrap() };
        package_linkage_impls(self.0).downcast()
    }
}

impl MonoLinkageLibraries {
    pub(super) fn generate(target_path: LinktimeTargetPath, db: &::salsa::Db) -> Result<Self, ()> {
        match target_path.transpile_to_fs_full(target_path.transpilation_setup(db), db) {
            Ok(()) => (),
            Err(_) => todo!(),
        };
        let rust_workspace_dir = target_path.rust_workspace_abs_dir(db);
        let all_packages: HashMap<PathBuf, (TaskJarIndex, PackagePath)> = HashMap::from_iter(
            target_path
                .all_packages(db)
                .unwrap()
                .iter()
                .copied()
                .enumerate()
                .map(|(i, package_path)| {
                    let linkages_cargo_toml_path = rust_workspace_dir
                        .join(package_path.name(db).data(db))
                        .join("linkages/Cargo.toml");
                    (
                        linkages_cargo_toml_path,
                        (TaskJarIndex::from_index(i), package_path),
                    )
                }),
        );
        let cdylibs: VecPairMap<PackagePath, Cdylib> = compile_workspace(
            target_path.rust_workspace_manifest_path(db),
            |compilation| unsafe {
                VecMap::from_iter_assuming_no_repetitions_unchecked(
                    compilation
                        .cdylibs
                        .iter()
                        .enumerate()
                        .map(|(i, unit_output)| {
                            let (jar_index, package_path) =
                                all_packages[unit_output.unit.pkg.manifest_path()];
                            (
                                package_path,
                                Cdylib(jar_index, Library::new(unit_output.path.clone()).unwrap()),
                            )
                        }),
                )
            },
        )?;
        assert_eq!(cdylibs.len(), all_packages.len());
        Ok(Self { cdylibs })
    }
}

#[test]
fn generate_linkage_storage_works() {
    use husky_dev_comptime::db::DevComptimeDb;
    use husky_print_utils::*;
    use salsa::DebugWithDb;
    let mut db = DevComptimeDb::default();
    db.vfs_plain_test(
        |db, package_path: PackagePath| {
            MonoLinkageLibraries::generate(LinktimeTargetPath::new_package(package_path, db), db)
                .unwrap();
        },
        &VfsTestConfig::new("generate_linkage_storage")
            .with_vfs_test_domains_config(VfsTestDomainsConfig::ExamplesOnly),
    );
}
