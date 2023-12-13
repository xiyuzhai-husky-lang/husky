use super::*;
use husky_cargo_utils::compile_workspace;
use husky_corgi_config::transpilation_setup::HasTranspilationSetup;
use husky_manifest::{has_manifest::HasPackageManifest, HasAllPackages};
use husky_rust_transpilation::{db::RustTranspilationJar, transpile_to_fs::TranspileToFsFull};
use husky_task::IsTask;
use husky_vfs::{linktime_target_path::LinktimeTargetPathData, PackagePath};
use libloading::Library;
use std::path::PathBuf;
use vec_like::{VecMap, VecPairMap, VecSet};

pub struct MonoLinkageLibraries {
    cdylibs: VecPairMap<PackagePath, Library>,
}

impl MonoLinkageLibraries {
    pub(super) fn generate(target_path: LinktimeTargetPath, db: &::salsa::Db) -> Self {
        match target_path.transpile_to_fs_full(target_path.transpilation_setup(db), db) {
            Ok(()) => (),
            Err(_) => todo!(),
        };
        let rust_workspace_dir = target_path.rust_workspace_abs_dir(db);
        let all_packages: HashMap<PathBuf, PackagePath> = HashMap::from_iter(
            target_path
                .all_packages(db)
                .unwrap()
                .iter()
                .map(|&package_path| {
                    let linkages_cargo_toml_path = rust_workspace_dir
                        .join(package_path.name(db).data(db))
                        .join("linkages/Cargo.toml");
                    (linkages_cargo_toml_path, package_path)
                }),
        );
        let cdylibs: VecPairMap<PackagePath, Library> = compile_workspace(
            target_path.rust_workspace_manifest_path(db),
            |compilation| unsafe {
                VecMap::from_iter_assuming_no_repetitions_unchecked(compilation.cdylibs.iter().map(
                    |unit_output| {
                        (
                            all_packages[unit_output.unit.pkg.manifest_path()],
                            Library::new(unit_output.path.clone()).unwrap(),
                        )
                    },
                ))
            },
        );
        Self { cdylibs }
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
            let libraries = MonoLinkageLibraries::generate(
                LinktimeTargetPath::new_package(package_path, db),
                db,
            );
            for (_, library) in libraries.cdylibs.iter() {
                unsafe {
                    let _: libloading::Symbol<fn()> = library.get(b"linkage_impls").unwrap();
                }
            }
        },
        &VfsTestConfig::new("generate_linkage_storage")
            .with_vfs_test_domains_config(VfsTestDomainsConfig::ExamplesOnly),
    );
}
