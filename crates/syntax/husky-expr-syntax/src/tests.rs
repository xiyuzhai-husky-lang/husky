use crate::*;
use husky_entity_tree::EntityTreeJar;
use husky_expect_test_utils::expect_test_husky_to_rust;
use husky_source_path::SourcePathJar;
use husky_symbol_syntax::{SymbolContext, SymbolSheet};
use husky_token_text::TokenTextJar;
use husky_tokenize::Tokenize;
use husky_vfs::{HasFileCache, HasWatcherPlace, VfsJar};
use place::SingleAssignPlace;
use salsa::{Database, ParallelDatabase};

#[test]
fn it_works() {
    expect_test_husky_to_rust("", &test_parse_exprs_debug);

    fn test_parse_exprs_debug(text: &str) -> String {
        let db = MimicDB::default();
        let tokens = db.tokenize_line(text);
        let mut arena = ExprArena::new();
        // ad hoc; todo: preludes
        let mut symbols = SymbolContext::new(&[]);
        parse_expr(&db, &tokens, &mut symbols, &mut arena);
        format!("{:#?}", arena)
    }
}

#[salsa::db(
    IdentifierJar,
    VfsJar,
    SourcePathJar,
    TextJar,
    TokenTextJar,
    EntityTreeJar
)]
#[derive(Default)]
struct MimicDB {
    storage: salsa::Storage<Self>,
}

impl Database for MimicDB {}

impl ParallelDatabase for MimicDB {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        todo!()
    }
}

impl HasFileCache for MimicDB {
    fn cache(&self) -> &husky_vfs::HuskyFileCache {
        todo!()
    }
}

impl HasWatcherPlace for MimicDB {
    fn watcher_place_mut(&mut self) -> &mut SingleAssignPlace<husky_vfs::VfsWatcher> {
        todo!()
    }

    fn watcher_place(&self) -> &SingleAssignPlace<husky_vfs::VfsWatcher> {
        todo!()
    }
}
