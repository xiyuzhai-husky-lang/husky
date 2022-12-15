use husky_entity_path::{EntityPathDb, EntityPathJar};
use husky_package_path::{PackagePathDb, PackagePathJar};
use husky_toolchain::ToolchainJar;
use husky_word::WordJar;

use crate::*;

#[salsa::db(WordJar, ToolchainJar, PackagePathJar, EntityPathJar, SourcePathJar)]
#[derive(Default)]
pub struct MimicDB {
    storage: salsa::Storage<Self>,
    source_path_config: SourcePathConfigMimic,
}

impl salsa::Database for MimicDB {}

impl HasSourcePathConfig for MimicDB {
    fn source_path_config(&self) -> &SourcePathConfig {
        &self.source_path_config
    }
}
