use crate::*;
use expect_test::expect_file;
use husky_entity_path::{EntityPathDb, EntityPathJar};
use husky_entity_tree::EntityTreeJar;
use husky_package_path::{PackagePathDb, PackagePathJar};
use husky_print_utils::p;
use husky_source_path::{
    HasSourcePathConfig, SourcePathConfig, SourcePathConfigMimic, SourcePathData, SourcePathDb,
    SourcePathJar,
};
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;
use husky_toolchain::ToolchainJar;
use husky_vfs::VfsJar;
use husky_word::{WordDb, WordJar};
use salsa::{Database, ParallelDatabase, Snapshot};
use std::{borrow::Cow, sync::Arc};

#[salsa::db(
    WordJar,
    ToolchainJar,
    PackagePathJar,
    EntityPathJar,
    EntityTreeJar,
    VfsJar,
    SourcePathJar,
    TokenJar,
    AstJar
)]
#[derive(Default)]
struct MimicDB {
    storage: salsa::Storage<Self>,
    source_path_config: SourcePathConfigMimic,
}

impl HasSourcePathConfig for MimicDB {
    fn source_path_config(&self) -> &SourcePathConfig {
        &self.source_path_config
    }
}

impl Database for MimicDB {}

impl ParallelDatabase for MimicDB {
    fn snapshot(&self) -> Snapshot<Self> {
        Snapshot::new(MimicDB {
            storage: self.storage.snapshot(),
            source_path_config: self.source_path_config.clone(),
        })
    }
}

#[test]
fn library_ast_works() {
    let db = MimicDB::default();
    let package_path_menu = db.package_path_menu();
    let entity_path_menu = db.entity_path_menu();

    macro_rules! t {
        ($($module:ident),*) => {
            $(
                expect_file![format!("../tests/single/{}_ast_sheet.txt", stringify!($module))]
                    .assert_eq(&format!("{:#?}", db.ast_sheet(entity_path_menu.$module())))
            );*
        }
    }

    t!(core, core_basic, core_num, std);
}
