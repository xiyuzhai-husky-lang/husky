use husky_toolchain::Toolchain;
use husky_word::Identifier;
use semver::Version;
use std::path::PathBuf;
use url::Url;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PackagePath {
    Builtin {
        ident: Identifier,
        toolchain: Toolchain,
    },
    Global {
        ident: Identifier,
        version: Version,
    },
    Local(PathBuf),
    Git(Url),
}
