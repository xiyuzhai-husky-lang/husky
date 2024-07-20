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

pub struct MonoLinketsLibrary {
    pub cdylib: Cdylib,
}

#[salsa::derive_debug_with_db]
pub struct Cdylib(Library);

impl Cdylib {
    pub(crate) fn linket_impls<LinketImpl: IsLinketImpl>(&self) -> Vec<LinketImpl> {
        let package_linket_impls: libloading::Symbol<fn() -> AnyLinketImpls> =
            unsafe { self.0.get(b"linket_impls").unwrap() };
        package_linket_impls().downcast()
    }
}

impl MonoLinketsLibrary {
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
        let cdylib: Cdylib = compile_workspace(
            target_path.rust_workspace_manifest_path(db),
            |compilation| unsafe {
                assert_eq!(compilation.cdylibs.len(), 1, "expect only one dylib");
                let cdylib = &compilation.cdylibs[0];
                Cdylib(Library::new(cdylib.path.clone()).unwrap())
            },
        )?;
        Ok(Self { cdylib })
    }
}

#[test]
fn generate_linket_storage_works() {
    use husky_dev_comptime::db::DevComptimeDb;

    DevComptimeDb::vfs_plain_test(
        |db, package_path: PackagePath| {
            MonoLinketsLibrary::generate(LinktimeTargetPath::new_package(package_path, db), db)
                .unwrap();
        },
        &VfsTestConfig::new(
            "generate_linket_storage",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::LINKTIME,
        ),
    );
}
