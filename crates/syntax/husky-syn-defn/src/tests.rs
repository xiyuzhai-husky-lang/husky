pub(crate) use husky_ast::test_utils::*;

use crate::*;
use husky_ast::AstJar;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_coword::CowordJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::EntitySynTreeJar;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;
use salsa::Database;

#[salsa::db(
    CowordJar,
    VfsJar,
    EntityPathJar,
    TermPreludeJar,
    TokenJar,
    AstJar,
    EntitySynTreeJar,
    TomlTokenJar,
    TomlAstJar,
    ManifestAstJar,
    CorgiConfigJar,
    CorgiConfigAstJar,
    ManifestJar,
    SynDeclJar,
    SynExprJar,
    SynDefnJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl Database for DB {}
