use super::*;
use husky_any_linket_impls::{LinketImplsGetter, LINKET_IMPLS_GETTER_IDENT};
use husky_cargo_utils::compile::compile_workspace;
use husky_corgi_config::transpilation_setup::HasTranspilationSetup;
use husky_item_path_interface::ItemPathIdInterface;
use husky_linket::linket::{target_linket_item_path_id_interfaces, target_linkets};
use husky_linket_impl::linket_impls::LinketImpls;
use husky_manifest::helpers::upstream::HasAllUpstreamPackages;
use husky_rust_transpilation::transpile_to_fs::TranspileToFsFull;
use husky_vfs::path::package_path::PackagePath;
use libloading::Library;
use std::path::PathBuf;
use vec_like::{VecMap, VecPairMap};

pub struct MonoLinketsLibrary<LinketImpl: IsLinketImpl> {
    pub cdylib: Cdylib,
    pub linket_impls: LinketImpls<LinketImpl>,
}

#[salsa::derive_debug_with_db]
pub struct Cdylib(Library);

impl Cdylib {
    fn linket_impls<LinketImpl: IsLinketImpl>(
        &self,
        item_path_id_interfaces: &[Option<ItemPathIdInterface>],
    ) -> LinketImpls<LinketImpl> {
        let linket_impls_getter: libloading::Symbol<LinketImplsGetter> =
            unsafe { self.0.get(LINKET_IMPLS_GETTER_IDENT).unwrap() };
        linket_impls_getter(item_path_id_interfaces).downcast()
    }
}

impl<LinketImpl: IsLinketImpl> MonoLinketsLibrary<LinketImpl> {
    pub(super) fn new(target_path: LinktimeTargetPath, db: &::salsa::Db) -> Result<Self, ()> {
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
        let item_path_id_interfaces = target_linket_item_path_id_interfaces(db, target_path);
        let linket_impls = cdylib.linket_impls(item_path_id_interfaces);
        Ok(Self {
            cdylib,
            linket_impls,
        })
    }

    pub fn init(&mut self, runtime: &'static dyn IsDevRuntimeDyn<LinketImpl>) {
        unsafe { self.linket_impls.set_dev_eval_context(runtime) };
    }

    pub fn release(&mut self) {
        unsafe { self.linket_impls.unset_dev_eval_context() };
    }
}

#[test]
fn new_mono_linkets_libary_works() {
    use husky_dev_comptime::db::DevComptimeDb;
    use husky_standard_linket_impl::StandardLinketImpl;

    DevComptimeDb::vfs_plain_test(
        |db, package_path: PackagePath| {
            MonoLinketsLibrary::<StandardLinketImpl>::new(
                LinktimeTargetPath::new_package(package_path, db),
                db,
            )
            .unwrap();
        },
        &VfsTestConfig::new(
            "generate_linket_storage",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::LINKTIME,
        ),
    );
}

/// # getters
impl<LinketImpl: IsLinketImpl> MonoLinketsLibrary<LinketImpl> {
    pub fn linket_impls(&self) -> &LinketImpls<LinketImpl> {
        &self.linket_impls
    }
}

/// # actions
impl<LinketImpl: IsLinketImpl> MonoLinketsLibrary<LinketImpl> {}
