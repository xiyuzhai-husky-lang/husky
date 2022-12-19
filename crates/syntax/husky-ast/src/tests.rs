pub(crate) use expect_test::expect_file;
pub(crate) use husky_absolute_path::AbsolutePath;
pub(crate) use husky_vfs_test_utils::VfsTestSupport;

use crate::*;
use husky_entity_path::{EntityPathData, EntityPathDb, EntityPathJar};
use husky_package_path::{CrateKind, PackagePathData, PackagePathDb, PackagePathJar};
use husky_token::TokenJar;
use husky_toolchain::*;
use husky_vfs::*;
use husky_word::{WordDb, WordJar};
use salsa::{Database, DebugWithDb, ParallelDatabase, Snapshot};
use std::{borrow::Cow, sync::Arc};

#[salsa::db(
    WordJar,
    ToolchainJar,
    PackagePathJar,
    EntityPathJar,
    VfsJar,
    TokenJar,
    AstJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
    source_path_config: VfsConfigMimic,
}

impl HasVfsConfig for DB {
    fn vfs_config(&self) -> &VfsConfig {
        &self.source_path_config
    }
}

impl Database for DB {}

impl ParallelDatabase for DB {
    fn snapshot(&self) -> Snapshot<Self> {
        Snapshot::new(DB {
            storage: self.storage.snapshot(),
            source_path_config: self.source_path_config.clone(),
        })
    }
}
