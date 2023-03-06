use crate::*;
use husky_ast::AstJar;
use husky_decl::DeclJar;
use husky_entity_path::{EntityPathJar, ModuleItemPath, TypePath};
use husky_entity_tree::{EntityTreeDb, EntityTreeJar};
use husky_expr::ExprJar;
use husky_manifest::ManifestJar;
use husky_print_utils::p;
use husky_raw_term::RawTermJar;
use husky_signature::SignatureJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
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
    TermPreludeJar,
    RawTermJar,
    SignatureJar,
    RawTypeJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<DB>,
}

impl salsa::Database for DB {}
