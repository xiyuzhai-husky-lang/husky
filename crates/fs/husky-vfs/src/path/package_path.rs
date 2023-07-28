use super::*;

use husky_coword::Ident;
use salsa::DebugWithDb;
use std::path::Path;
use url::Url;

/// deprecated
#[test]
fn package_ident_works() {
    let db = DB::default();
    let _toolchain = db.dev_toolchain().unwrap();
    let ident_menu = db.coword_menu();
    let path_menu = db.dev_path_menu().unwrap();
    assert_eq!(path_menu.core_package().ident(&db), ident_menu.core_ident());
    assert_eq!(path_menu.std_package().ident(&db), ident_menu.std_ident());
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
#[salsa::debug_with_db(db = VfsDb, jar = VfsJar)]
pub enum PackagePathSource {
    Toolchain,
    Registry {
        registry_path: RegistryPath,
        version: semver::Version,
    },
    Local {
        path: DiffPath,
    },
    Git {
        url: Url,
    },
}

#[salsa::interned(jar = VfsJar, db = VfsDb, constructor = new_inner)]
pub struct PackagePath {
    pub toolchain: Toolchain,
    pub name: Name,
    #[return_ref]
    pub data: PackagePathSource,
}

impl PackagePath {
    pub fn new_local_package(
        db: &dyn VfsDb,
        toolchain: Toolchain,
        name: Name,
        path: &Path,
    ) -> VfsResult<Self> {
        Ok(PackagePath::new_inner(
            db,
            toolchain,
            name,
            PackagePathSource::Local {
                path: DiffPath::try_new(db, path)?,
            },
        ))
    }

    pub fn new_toolchain_package(db: &dyn VfsDb, toolchain: Toolchain, name: Name) -> Self {
        PackagePath::new_inner(db, toolchain, name, PackagePathSource::Toolchain)
    }

    pub fn new_registry_package(
        db: &dyn VfsDb,
        toolchain: Toolchain,
        name: Name,
        registry_path: RegistryPath,
        version: semver::Version,
    ) -> Self {
        PackagePath::new_inner(
            db,
            toolchain,
            name,
            PackagePathSource::Registry {
                registry_path,
                version,
            },
        )
    }

    pub fn ident(self, db: &dyn VfsDb) -> Ident {
        self.name(db).ident(db)
    }

    pub fn dir(self, db: &dyn VfsDb) -> VfsResult<DiffPath> {
        package_dir(db, self)
    }

    pub fn manifest_path(self, db: &dyn VfsDb) -> VfsResult<ManifestPath> {
        package_manifest_path(db, self)
    }

    pub fn lib_crate(self, db: &dyn VfsDb) -> CratePath {
        CratePath::new(db, self, CrateKind::Library)
    }

    pub fn lib_module(self, db: &dyn VfsDb) -> ModulePath {
        ModulePath::new_root(db, self.lib_crate(db))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegistryPath(DiffPath);

impl RegistryPath {
    pub fn new(path: DiffPath) -> Self {
        Self(path)
    }

    pub fn path(self) -> DiffPath {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ManifestPath(DiffPath);

impl ManifestPath {
    pub fn path(self) -> DiffPath {
        self.0
    }
}

#[salsa::tracked(jar = VfsJar)]
pub(crate) fn package_dir(db: &dyn VfsDb, package: PackagePath) -> VfsResult<DiffPath> {
    match package.data(db) {
        PackagePathSource::Toolchain => DiffPath::try_new(
            db,
            &package
                .toolchain(db)
                .library_path(db)
                .join(package.name(db).data(db)),
        ),
        PackagePathSource::Registry {
            registry_path,
            version,
            ..
        } => DiffPath::try_new(
            db,
            registry_path.path().path(db).join(format!(
                "{}-{}.{}.{}",
                package.name(db).data(db),
                version.major,
                version.minor,
                version.patch
            )),
        ),
        PackagePathSource::Local { path } => Ok(path.clone()),
        PackagePathSource::Git { .. } => todo!(),
    }
}

#[salsa::tracked(jar = VfsJar)]
pub(crate) fn package_manifest_path(
    db: &dyn VfsDb,
    package: PackagePath,
) -> VfsResult<ManifestPath> {
    Ok(ManifestPath(DiffPath::try_new(
        db,
        package.dir(db)?.path(db).join("Corgi.toml"),
    )?))
}
