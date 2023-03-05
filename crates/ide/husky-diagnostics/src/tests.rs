use crate::*;
use husky_ast::AstJar;
use husky_decl::DeclJar;
use husky_defn::DefnJar;
use husky_entity_path::{EntityPathDb, EntityPathJar};
use husky_entity_tree::EntityTreeJar;
use husky_expr::ExprJar;
use husky_expr_ty::ExprTypeJar;
use husky_manifest::ManifestJar;
use husky_precise_term::PreciseTermJar;
use husky_raw_term::RawTermJar;
use husky_raw_ty::RawTypeJar;
use husky_signature::SignatureJar;
use husky_term::TermJar;
use husky_token::TokenJar;
use husky_ty::TypeJar;
use husky_valid_term::ValidTermJar;
use husky_word::WordJar;
use husky_valid_ty::ValidTypeJar;

#[salsa::db(
    VfsJar,
    WordJar,
    TokenJar,
    EntityPathJar,
    ManifestJar,
    AstJar,
    EntityTreeJar,
    DeclJar,
    DefnJar,
    ExprJar,
    DiagnosticsJar,
    RawTermJar,
    RawTypeJar,
    PreciseTermJar,
    ValidTermJar,
    ValidTypeJar,
    TermJar,
    TypeJar,
    SignatureJar,
    ExprTypeJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}
