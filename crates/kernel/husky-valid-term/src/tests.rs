use crate::*;
use husky_ast::AstJar;
use husky_decl::DeclJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::{EntityTreeDb, EntityTreeJar};
use husky_expr::ExprJar;
use husky_manifest::ManifestJar;
use husky_precise_term::PreciseTermJar;
use husky_raw_term::RawTermJar;
use husky_raw_ty::RawTypeJar;
use husky_signature::SignatureJar;
use husky_token::TokenJar;
use husky_vfs::VfsJar;
use husky_vfs::*;
use husky_word::WordJar;

#[salsa::db(
    WordJar,
    VfsJar,
    EntityPathJar,
    TokenJar,
    AstJar,
    EntityTreeJar,
    ManifestJar,
    ExprJar,
    DeclJar,
    RawTermJar,
    SignatureJar,
    RawTypeJar,
    PreciseTermJar,
    ValidTermJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<DB>,
}

impl salsa::Database for DB {}
