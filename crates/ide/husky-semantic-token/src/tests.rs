use crate::*;
use husky_ast::AstJar;
use husky_decl::DeclJar;
use husky_defn::DefnJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::EntityTreeJar;
use husky_manifest::ManifestJar;
use husky_word::WordJar;

#[salsa::db(
    VfsJar,
    WordJar,
    TokenJar,
    TokenInfoJar,
    EntityPathJar,
    ManifestJar,
    AstJar,
    EntityTreeJar,
    SemanticTokenJar,
    DeclJar,
    DefnJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

#[test]
fn semantic_tokens_works() {
    DB::expect_test_probable_modules_debug("semantic_tokens", SemanticTokenDb::semantic_tokens_ext)
}
