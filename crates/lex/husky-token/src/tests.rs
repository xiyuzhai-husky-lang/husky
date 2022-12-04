use crate::*;
use expect_test::expect_file;
use husky_expect_test_utils::*;
use husky_package_path::{PackagePathDb, PackagePathJar};
use husky_print_utils::p;
use husky_source_path::{
    HasSourcePathConfig, SourcePathConfig, SourcePathConfigMimic, SourcePathJar,
};
use husky_toolchain::ToolchainJar;
use husky_vfs::VfsJar;
use husky_word::{WordDb, WordJar};
use salsa::{Database, ParallelDatabase, Snapshot, Storage};
use std::{borrow::Cow, sync::Arc};

#[salsa::db(WordJar, ToolchainJar, PackagePathJar, TokenJar, VfsJar, SourcePathJar)]
#[derive(Default)]
struct MimicDB {
    storage: Storage<Self>,
    source_path_config: SourcePathConfigMimic,
}

impl HasSourcePathConfig for MimicDB {
    fn source_path_config(&self) -> &SourcePathConfig {
        &self.source_path_config
    }
}

impl Database for MimicDB {}

impl salsa::ParallelDatabase for MimicDB {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        todo!()
    }
}

#[test]
fn tokenize_works() {
    expect_test_husky_to_rust("", &tokenize_debug);

    fn tokenize_debug(text: &str) -> String {
        format!("{:#?}", MimicDB::default().tokenize(text))
    }
}

#[test]
fn tokenize_library() {}
