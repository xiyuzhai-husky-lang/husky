use super::*;

use husky_minimal_toml_utils::find_package_name_in_toml;
use husky_word::Ident;
use salsa::DebugWithDb;
use std::path::Path;
use url::Url;

#[salsa::tracked(jar = VfsJar)]
pub(crate) fn package_ident(db: &dyn VfsDb, package_path: PackagePath) -> VfsResult<Ident> {
    let toml_content = db.package_manifest_content(package_path)?;
    let name = find_package_name_in_toml(toml_content)?;
    Ident::from_owned(db, husky_word::dash_to_snake(name)).ok_or(VfsError::PackageIdent)
}

/// deprecated
#[test]
fn package_ident_works() {
    let db = DB::default();
    let _toolchain = db.dev_toolchain().unwrap();
    let ident_menu = db.word_menu();
    let path_menu = db.dev_path_menu().unwrap();
    assert_eq!(
        path_menu.core_package().ident(&db),
        Ok(ident_menu.core_ident())
    );
    assert_eq!(
        path_menu.std_package().ident(&db),
        Ok(ident_menu.std_ident())
    );
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
#[salsa::derive_debug_with_db(db = VfsDb, jar = VfsJar)]
pub enum PackagePathSource {
    Toolchain {
        name: Name,
    },
    Registry {
        registry_path: RegistryPath,
        name: Name,
        version: semver::Version,
    },
    Local {
        path: DiffPath,
    },
    Git {
        url: Url,
    },
}

#[salsa::interned(jar = VfsJar, db = VfsDb)]
pub struct PackagePath {
    pub toolchain: Toolchain,
    #[return_ref]
    pub data: PackagePathSource,
}

impl PackagePath {
    pub fn new_local(db: &dyn VfsDb, toolchain: Toolchain, path: &Path) -> VfsResult<Self> {
        Ok(PackagePath::new(
            db,
            toolchain,
            PackagePathSource::Local {
                path: DiffPath::try_new(db, path)?,
            },
        ))
    }

    pub fn new_toolchain_package(db: &dyn VfsDb, toolchain: Toolchain, name: Name) -> Self {
        PackagePath::new(db, toolchain, PackagePathSource::Toolchain { name })
    }

    pub fn ident(self, db: &dyn VfsDb) -> VfsResult<Ident> {
        package_ident(db, self)
    }

    pub fn dir(self, db: &dyn VfsDb) -> VfsResult<DiffPath> {
        package_dir(db, self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegistryPath(DiffPath);

#[salsa::tracked(jar = VfsJar)]
pub(crate) fn package_dir(db: &dyn VfsDb, package: PackagePath) -> VfsResult<DiffPath> {
    match package.data(db) {
        PackagePathSource::Toolchain { name } => DiffPath::try_new(
            db,
            &package.toolchain(db).library_path(db).join(name.data(db)),
        ),
        PackagePathSource::Registry { name, version, .. } => todo!(),
        PackagePathSource::Local { path } => Ok(path.clone()),
        PackagePathSource::Git { .. } => todo!(),
    }
}
