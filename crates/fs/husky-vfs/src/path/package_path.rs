use super::*;

use husky_coword::Ident;

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
#[salsa::debug_with_db(db = VfsDb)]
pub enum PackagePathSource {
    Library,
    Registry {
        registry_path: RegistryPath,
        version: semver::Version,
    },
    Local {
        path: VirtualPath,
    },
    Git {
        url: Url,
    },
}

#[salsa::interned(jar = VfsJar, db = VfsDb, constructor = new_inner)]
pub struct PackagePath {
    pub toolchain: Toolchain,
    pub name: Kebab,
    #[return_ref]
    pub data: PackagePathSource,
}

impl PackagePath {
    /// if name is `core` or `std` make sure that the package is the toolchain one
    pub fn new_local_or_toolchain_package(
        db: &dyn VfsDb,
        toolchain: Toolchain,
        name: Kebab,
        path: &Path,
    ) -> VfsResult<Self> {
        match name.data(db) {
            "core" | "std" => {
                debug_assert_eq!(
                    std::fs::canonicalize(path.parent().unwrap()).unwrap(),
                    std::fs::canonicalize(toolchain.library_path(db)).unwrap()
                );
                Ok(Self::new_toolchain_package(db, toolchain, name))
            }
            _ => Ok(PackagePath::new_inner(
                db,
                toolchain,
                name,
                PackagePathSource::Local {
                    path: VirtualPath::try_new(db, path)?,
                },
            )),
        }
    }

    pub fn new_toolchain_package(db: &dyn VfsDb, toolchain: Toolchain, name: Kebab) -> Self {
        PackagePath::new_inner(db, toolchain, name, PackagePathSource::Library)
    }

    pub fn new_registry_package(
        db: &dyn VfsDb,
        toolchain: Toolchain,
        name: Kebab,
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

    pub fn dir(self, db: &dyn VfsDb) -> VfsResult<VirtualPath> {
        package_dir(db, self)
    }

    pub fn manifest_path(self, db: &dyn VfsDb) -> VfsResult<ManifestPath> {
        package_manifest_path(db, self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegistryPath(VirtualPath);

impl RegistryPath {
    pub fn new(path: VirtualPath) -> Self {
        Self(path)
    }

    pub fn path(self) -> VirtualPath {
        self.0
    }
}

#[salsa::tracked(jar = VfsJar)]
pub(crate) fn package_dir(db: &dyn VfsDb, package: PackagePath) -> VfsResult<VirtualPath> {
    match package.data(db) {
        PackagePathSource::Library => VirtualPath::try_new(
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
        } => VirtualPath::try_new(
            db,
            registry_path.path().data(db).join(format!(
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ManifestPath(VirtualPath);

impl ManifestPath {
    pub fn path(self) -> VirtualPath {
        self.0
    }
}

#[salsa::tracked(jar = VfsJar)]
pub(crate) fn package_manifest_path(
    db: &dyn VfsDb,
    package: PackagePath,
) -> VfsResult<ManifestPath> {
    Ok(ManifestPath(VirtualPath::try_new(
        db,
        package.dir(db)?.data(db).join("Corgi.toml"),
    )?))
}
