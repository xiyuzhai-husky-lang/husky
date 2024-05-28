#[path = "crate/lib.rs"]
pub mod lib;
#[path = "crate/main.rs"]
pub mod main;
#[path = "crate/requirements.rs"]
pub mod requirements;
#[path = "crate/task.rs"]
pub mod task;

use self::lib::*;
use self::main::*;
use self::requirements::*;
use self::task::*;
use super::*;
use husky_vfs::{CrateKind, CratePath};

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum CrateSynNodeDecl {
    Lib(LibCrateSynNodeDecl),
    Main(MainCrateSynNodeDecl),
    Requirements(RequirementsCrateSynNodeDecl),
    Task(TaskCrateSynNodeDecl),
}

impl HasSynNodeDecl for CratePath {
    type NodeDecl = CrateSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a salsa::Db) -> Self::NodeDecl {
        crate_syn_node_decl(db, self)
    }
}

#[salsa::tracked]
fn crate_syn_node_decl(db: &::salsa::Db, crate_path: CratePath) -> CrateSynNodeDecl {
    let parser = CrateDeclParser::new(db, crate_path);
    parser.parse_crate_syn_node_decl()
}

impl<'db> CrateDeclParser<'db> {
    fn parse_crate_syn_node_decl(&self) -> CrateSynNodeDecl {
        let db = self.db();
        match self.crate_path().kind(db) {
            CrateKind::Lib => self.parse_lib_crate_syn_node_decl(),
            CrateKind::Main => self.parse_lib_crate_syn_node_decl(),
            CrateKind::Requirements => self.parse_lib_crate_syn_node_decl(),
            CrateKind::Task => self.parse_lib_crate_syn_node_decl(),
            CrateKind::Bin(_) => todo!(),
            CrateKind::IntegratedTest(_) => todo!(),
            CrateKind::Example => todo!(),
        }
    }
}

#[test]
fn crate_syn_node_decl_works() {
    DB::ast_expect_test_debug_with_db(
        crate_syn_node_decl,
        &AstTestConfig::new(
            "crate_syn_node_decl",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SYNTAX,
        ),
    )
}
