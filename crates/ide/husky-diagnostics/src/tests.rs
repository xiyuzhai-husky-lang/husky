pub(crate) use husky_ast::test_utils::*;

use crate::*;
use husky_ast::AstJar;
use husky_decl::DeclJar;
use husky_defn::DefnJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::EntityTreeJar;
use husky_expr::ExprJar;
use husky_expr_ty::ExprTypeJar;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_raw_term::RawTermJar;
use husky_raw_ty::RawTypeJar;
use husky_signature::SignatureJar;
use husky_term::TermJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;
use husky_word::WordJar;

#[salsa::db(
    VfsJar,
    WordJar,
    TokenJar,
    EntityPathJar,
    TomlTokenJar,
    TomlAstJar,
    ManifestAstJar,
    ManifestJar,
    AstJar,
    EntityTreeJar,
    DeclJar,
    DefnJar,
    ExprJar,
    DiagnosticsJar,
    TermPreludeJar,
    RawTermJar,
    RawTypeJar,
    TermJar,
    SignatureJar,
    ExprTypeJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}
