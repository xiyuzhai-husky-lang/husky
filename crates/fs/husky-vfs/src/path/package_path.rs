use super::*;

use husky_word::Identifier;
use salsa::DebugWithDb;
use std::path::Path;
use url::Url;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PackagePathData {
    PublishedToolchain {
        ident: Identifier,
        toolchain: PublishedToolchain,
    },
    Global {
        ident: Identifier,
        version: semver::Version,
    },
    Local {
        path: AbsolutePath,
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
                path: AbsolutePath::try_new(db, path)?,
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
                PackagePath::new_local(db, toolchain, &library_path.join(ident.data(db)))
            }
        }
    }
}

impl DebugWithDb<dyn VfsDb + '_> for PackagePathData {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn VfsDb,
        _include_all_fields: bool,
    ) -> ::std::fmt::Result {
        match self {
            PackagePathData::PublishedToolchain { ident, toolchain } => f
                .debug_struct("Builtin")
                .field("ident", &ident.data(db))
                .field("toolchain", &toolchain.debug(db))
                .finish(),
            PackagePathData::Global { ident, ref version } => f
                .debug_struct("Glocal")
                .field("ident", &ident.data(db))
                .field("version", version)
                .finish(),
            PackagePathData::Local { path } => f.debug_struct("Local").field("path", path).finish(),
            PackagePathData::Git { url } => f.debug_struct("Git").field("url", url).finish(),
        }
    }
}
