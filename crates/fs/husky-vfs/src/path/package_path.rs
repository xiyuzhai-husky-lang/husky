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

#[test]
fn package_ident_works() {
    let db = DB::default();
    let _toolchain = db.dev_toolchain().unwrap();
    let word_menu = db.word_menu();
    let path_menu = db.dev_path_menu().unwrap();
    assert_eq!(path_menu.core_package().ident(&db), Ok(word_menu.core()));
    assert_eq!(path_menu.std_package().ident(&db), Ok(word_menu.std()));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PackagePathData {
    Toolchain {
        ident: Ident,
        toolchain: Toolchain,
    },
    Global {
        ident: Ident,
        version: semver::Version,
    },
    Local {
        path: DiffPath,
    },
    Git {
        url: Url,
    },
}

#[salsa::interned(jar = VfsJar, db = VfsDb, override_debug)]
pub struct PackagePath {
    pub toolchain: Toolchain,
    #[return_ref]
    pub data: PackagePathData,
}

impl PackagePath {
    pub fn new_local(db: &dyn VfsDb, toolchain: Toolchain, path: &Path) -> VfsResult<Self> {
        Ok(PackagePath::new(
            db,
            toolchain,
            PackagePathData::Local {
                path: DiffPath::try_new(db, path)?,
            },
        ))
    }

    pub fn new_toolchain_package(
        db: &dyn VfsDb,
        toolchain: Toolchain,
        ident: Ident,
    ) -> ToolchainResult<Self> {
        match toolchain.data(db) {
            ToolchainData::Published(_) => todo!(),
            ToolchainData::Local { library_path } => {
                PackagePath::new_local(db, toolchain, &library_path.data(db).join(ident.data(db)))
                    .map_err(|e| e.into())
            }
        }
    }

    pub fn ident(self, db: &dyn VfsDb) -> VfsResult<Ident> {
        package_ident(db, self)
    }
}

impl<Db: VfsDb + ?Sized> DebugWithDb<Db> for PackagePath {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DebugFormatLevel,
    ) -> ::std::fmt::Result {
        let db = <Db as salsa::DbWithJar<VfsJar>>::as_jar_db(db);
        f.debug_struct("PackagePath")
            .field("data", &self.data(db).debug(db))
            .finish()
    }
}

impl<Db: VfsDb + ?Sized> DebugWithDb<Db> for PackagePathData {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> ::std::fmt::Result {
        let db = <Db as salsa::DbWithJar<VfsJar>>::as_jar_db(db);
        match self {
            PackagePathData::Toolchain { ident, toolchain } => f
                .debug_struct("Builtin")
                .field("ident", &ident.data(db))
                .field("toolchain", &toolchain.debug_with(db, level.next()))
                .finish(),
            PackagePathData::Global { ident, ref version } => f
                .debug_struct("Glocal")
                .field("ident", &ident.data(db))
                .field("version", version)
                .finish(),
            PackagePathData::Local { path } => f
                .debug_struct("Local")
                .field("path", &path.debug_with(db, level.next()))
                .finish(),
            PackagePathData::Git { url } => f.debug_struct("Git").field("url", url).finish(),
        }
    }
}
