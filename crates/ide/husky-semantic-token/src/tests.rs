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
    TokenInferJar,
    EntityPathJar,
    ManifestJar,
    AstJar,
    EntityTreeJar,
    SemanticTokenJar,
    DeclJar,
    DefnJar
)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}
