use crate::*;
use husky_ast::AstJar;
use husky_decl::DeclJar;
use husky_defn::DefnJar;
use husky_entity_path::{EntityPathJar};
use husky_entity_tree::{EntityTreeJar};
use husky_expr::ExprJar;
use husky_expr_ty::ExprTypeJar;
use husky_manifest::ManifestJar;
use husky_raw_term::RawTermJar;
use husky_raw_ty::RawTypeJar;
use husky_signature::SignatureJar;
use husky_term::TermJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_vfs::*;
use husky_word::WordJar;

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
    TermPreludeJar,
    RawTermJar,
    SignatureJar,
    RawTypeJar,
    TermJar,
    ExprTypeJar,
    TokenInfoJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

#[test]
fn token_infer_sheet_works() {
    DB::default().vfs_expect_test_debug("token_infer_sheet", TokenInfoDb::token_info_sheet)
}
