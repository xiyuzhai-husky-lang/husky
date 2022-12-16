use crate::*;
use husky_package_path::PackagePathJar;
use husky_toolchain::ToolchainJar;
use husky_word::WordJar;

#[salsa::db(WordJar, ToolchainJar, PackagePathJar, EntityPathJar)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}
