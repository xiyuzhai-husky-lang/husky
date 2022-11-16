use husky_toolchain::Toolchain;
use husky_word::Identifier;
use semver::Version;
use std::path::PathBuf;
use url::Url;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PackagePathVariant {
    Builtin { toolchain: Toolchain },
    Global { version: Version },
    Local(PathBuf),
    Git(Url),
}
