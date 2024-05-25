use super::*;
use husky_vfs::PackagePath;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum PackageSynopsis {
    Lib,
    Main,
}

impl HasSynopsis for PackagePath {
    type Synopsis = PackageSynopsis;

    fn synopsis<'a>(self, db: &'a salsa::Db) -> ManifestResultRef<'a, &'a Self::Synopsis> {
        package_synopsis(db, self).as_ref()
    }
}

#[salsa::tracked(return_ref)]
fn package_synopsis(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> ManifestResult<PackageSynopsis> {
    let lib_crate_path = package_path.lib_crate_path(db);
    let main_crate_path = package_path.main_crate_path(db);
    let task_crate_path = package_path.task_crate_path(db).into_result_option()?;
    match (lib_crate_path, main_crate_path) {
        (None, None) => Err(OriginalManifestError::NoLibOrMainForPackage.into()),
        (None, Some(main_crate_path)) => Ok(PackageSynopsis::Main),
        (Some(lib_crate_path), None) => Ok(PackageSynopsis::Lib),
        (Some(lib_crate_path), Some(main_crate_path)) => todo!(),
    }
}

#[test]
fn package_synopsis_works() {
    DB::vfs_expect_test_debug_with_db(
        |db, package_path: PackagePath| package_path.synopsis(db),
        &VfsTestConfig::new(
            "package_synopsis",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::FS,
        ),
    )
}
