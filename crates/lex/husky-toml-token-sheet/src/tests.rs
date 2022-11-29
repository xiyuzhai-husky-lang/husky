use crate::*;
use husky_package_path::PackagePathJar;
use husky_toolchain::ToolchainJar;
use husky_word::WordJar;

#[salsa::db(WordJar, TomlTokenSheetJar, PackagePathJar, ToolchainJar)]
struct MimicDB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for MimicDB {}
