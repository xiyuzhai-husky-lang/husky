pub(crate) use husky_ast::test_utils::*;

use crate::*;
use husky_corgi_config::jar::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_entity_tree::EntityTreeJar;
use husky_manifest::jar::ManifestJar;
use husky_manifest_ast::jar::ManifestAstJar;
use husky_syn_decl::SynDeclJar;
use husky_syn_expr::jar::SynExprJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;

#[salsa::db(
    husky_coword::jar::CowordJar,
    husky_vfs::jar::VfsJar,
    husky_entity_path::jar::EntityPathJar,
    husky_token_data::jar::TokenDataJar,
    TokenJar,
    husky_ast::jar::AstJar,
    EntityTreeJar,
    husky_toml_token::jar::TomlTokenJar,
    TomlAstJar,
    ManifestAstJar,
    CorgiConfigJar,
    CorgiConfigAstJar,
    ManifestJar,
    SynExprJar,
    SynDeclJar,
    husky_term_prelude::jar::TermPreludeJar,
    husky_dec_term::jar::DecTermJar,
    DecSignatureJar,
    DeclarativeTypeJar
)]
#[derive(Default)]
pub(crate) struct DB;
