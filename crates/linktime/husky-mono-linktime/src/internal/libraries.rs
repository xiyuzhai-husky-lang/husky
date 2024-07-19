use super::*;
use husky_cargo_utils::compile::compile_workspace;
use husky_corgi_config::transpilation_setup::HasTranspilationSetup;
use husky_linket_impl::AnyLinketImpls;
use husky_manifest::helpers::upstream::HasAllUpstreamPackages;
use husky_rust_transpilation::transpile_to_fs::TranspileToFsFull;

use husky_devsoul_interface::HuskyJarIndex;
use husky_vfs::path::package_path::PackagePath;
use libloading::Library;
use std::path::PathBuf;
use vec_like::{VecMap, VecPairMap};

pub struct MonoLinketLibraries {
    pub cdylibs: VecPairMap<PackagePath, Cdylib>,
}

#[salsa::derive_debug_with_db]
pub struct Cdylib(HuskyJarIndex, Library);

impl Cdylib {
    pub(crate) fn linket_impls<LinketImpl: IsLinketImpl>(&self) -> Vec<LinketImpl> {
        let package_linket_impls: libloading::Symbol<fn(HuskyJarIndex) -> AnyLinketImpls> =
            unsafe { self.1.get(b"linket_impls").unwrap() };
        package_linket_impls(self.0).downcast()
    }
}

impl MonoLinketLibraries {
    pub(super) fn generate(target_path: LinktimeTargetPath, db: &::salsa::Db) -> Result<Self, ()> {
        // useful for debugging
        match std::env::var("SKIP_COMPILATION") {
            Ok(s) => {
                assert_eq!(s, "1");
            }
            Err(_) => {
                match target_path.transpile_to_fs_full(target_path.transpilation_setup(db), db) {
                    Ok(()) => (),
                    Err(_) => todo!(),
                };
            }
        }
        let rust_workspace_dir = target_path.rust_workspace_abs_dir(db);
        let all_packages: HashMap<PathBuf, (HuskyJarIndex, PackagePath)> = HashMap::from_iter(
            target_path
                .all_upstream_packages(db)
                .unwrap()
                .iter()
                .copied()
                .enumerate()
                .map(|(i, package_path)| {
                    let linkets_cargo_toml_path = rust_workspace_dir
                        .join(package_path.name(db).data(db))
                        .join("linkets/Cargo.toml");
                    (
                        linkets_cargo_toml_path,
                        (HuskyJarIndex::from_index(i), package_path),
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
                        .map(|(_i, unit_output)| {
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
fn generate_linket_storage_works() {
    use husky_dev_comptime::db::DevComptimeDb;

    DevComptimeDb::vfs_plain_test(
        |db, package_path: PackagePath| {
            MonoLinketLibraries::generate(LinktimeTargetPath::new_package(package_path, db), db)
                .unwrap();
        },
        &VfsTestConfig::new(
            "generate_linket_storage",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::LINKTIME,
        ),
    );
}
