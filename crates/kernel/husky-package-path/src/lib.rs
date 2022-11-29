mod builtin;
mod db;
mod jar;

pub use builtin::*;
pub use db::*;
pub use jar::*;

use husky_identifier::Identifier;
use husky_toolchain::Toolchain;
use semver::Version;
use std::path::PathBuf;
use url::Url;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PackagePathData {
    Builtin { toolchain: Toolchain },
    Global { version: Version },
    Local(PathBuf),
    Git(Url),
}

#[salsa::interned(jar = PackagePathJar)]
pub struct PackagePath {
    ident: Identifier,
    #[return_ref]
    data: PackagePathData,
}
