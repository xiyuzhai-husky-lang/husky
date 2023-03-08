use crate::*;
use husky_ast::AstJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::EntityTreeJar;
use husky_manifest::ManifestJar;
use husky_token::{TokenJar};
use husky_vfs::*;
use husky_word::WordJar;
use salsa::Database;

#[salsa::db(
    WordJar,
    VfsJar,
    EntityPathJar,
    TokenJar,
    AstJar,
    EntityTreeJar,
    ManifestJar,
    DeclJar,
    ExprJar,
    DefnJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl Database for DB {}
