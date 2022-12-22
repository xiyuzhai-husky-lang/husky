use super::*;

use husky_minimal_toml_utils::find_package_name_in_toml;
use husky_word::Identifier;
use salsa::DebugWithDb;
use std::path::Path;
use url::Url;

#[salsa::tracked(jar = VfsJar)]
pub(crate) fn package_ident(db: &dyn VfsDb, package_path: PackagePath) -> VfsResult<Identifier> {
    let toml_content = db.package_manifest_content(package_path)?;
    let name = find_package_name_in_toml(toml_content)?;
    Identifier::from_owned(db, husky_word::dash_to_snake(name)).ok_or(VfsError::PackageIdent)
}

#[test]
fn package_ident_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain();
    let word_menu = db.word_menu();
    let path_menu = db.path_menu(toolchain).unwrap();
    assert_eq!(path_menu.core_package().ident(&db), Ok(word_menu.core()));
    assert_eq!(path_menu.std_package().ident(&db), Ok(word_menu.std()));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PackagePathData {
    Toolchain {
        ident: Identifier,
        toolchain: Toolchain,
    },
    Global {
        ident: Identifier,
        version: semver::Version,
    },
    Local {
        path: DiffPath,
    },
    Git {
        url: Url,
    },
}

#[salsa::interned(jar = VfsJar)]
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

    pub fn new_toolchain(
        db: &dyn VfsDb,
        toolchain: Toolchain,
        ident: Identifier,
    ) -> VfsResult<Self> {
        match toolchain.data(db) {
            ToolchainData::Published(_) => todo!(),
            ToolchainData::Local { library_path } => {
                PackagePath::new_local(db, toolchain, &library_path.data(db).join(ident.data(db)))
            }
        }
    }

    pub fn ident(self, db: &dyn VfsDb) -> VfsResult<Identifier> {
        package_ident(db, self)
    }
}

impl DebugWithDb<dyn VfsDb + '_> for PackagePathData {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn VfsDb,
        include_all_fields: bool,
    ) -> ::std::fmt::Result {
        match self {
            PackagePathData::Toolchain { ident, toolchain } => f
                .debug_struct("Builtin")
                .field("ident", &ident.data(db))
                .field("toolchain", &toolchain.debug_with(db, include_all_fields))
                .finish(),
            PackagePathData::Global { ident, ref version } => f
                .debug_struct("Glocal")
                .field("ident", &ident.data(db))
                .field("version", version)
                .finish(),
            PackagePathData::Local { path } => f
                .debug_struct("Local")
                .field("path", &path.debug_with(db, include_all_fields))
                .finish(),
            PackagePathData::Git { url } => f.debug_struct("Git").field("url", url).finish(),
        }
    }
}
