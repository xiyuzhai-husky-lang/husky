use super::*;
use husky_vfs::PackagePath;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum PackageSynopsis {
    Lib {
        lib_crate_path: CratePath,
        task_crate_path: Option<CratePath>,
    },
    Main {
        main_crate_path: CratePath,
        task_crate_path: CratePath,
    },
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
    let mut task_crate_path = package_path.task_crate_path(db).into_result_option()?;
    for dep in package_path.dependencies(db)? {
        let dependent_package_path = dep.package_path();
        match *dependent_package_path.synopsis(db)? {
            PackageSynopsis::Lib {
                task_crate_path: Some(inherited_task_crate_path),
                ..
            } if task_crate_path.is_none() => task_crate_path = Some(inherited_task_crate_path),
            PackageSynopsis::Lib {
                task_crate_path: Some(inherited_task_crate_path),
                ..
            } if let Some(task_crate_path) = task_crate_path => {
                Err(OriginalManifestError::ConflictingTasks {
                    inherited_task_crate_path,
                    dependent_package_path,
                    task_crate_path,
                })?
            }
            PackageSynopsis::Lib { .. } | PackageSynopsis::Main { .. } => (),
        }
    }
    match (lib_crate_path, main_crate_path) {
        (None, None) => Err(OriginalManifestError::NoLibOrMainForPackage.into()),
        (None, Some(main_crate_path)) => Ok(PackageSynopsis::Main {
            main_crate_path,
            task_crate_path: task_crate_path.ok_or(OriginalManifestError::NoTaskForMain)?,
        }),
        (Some(lib_crate_path), None) => Ok(PackageSynopsis::Lib {
            lib_crate_path,
            task_crate_path,
        }),
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
