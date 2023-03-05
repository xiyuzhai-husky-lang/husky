use crate::TermJar;
use husky_ast::AstJar;
use husky_decl::DeclJar;
use husky_entity_path::{EntityPathJar, ModuleItemPath, TypePath};
use husky_entity_tree::{EntityTreeDb, EntityTreeJar};
use husky_expr::ExprJar;
use husky_manifest::ManifestJar;
use husky_precise_term::PreciseTermJar;
use husky_raw_term::RawTermJar;
use husky_raw_ty::RawTypeJar;
use husky_signature::SignatureJar;
use husky_token::TokenJar;
use husky_valid_term::ValidTermJar;
use husky_valid_ty::ValidTypeJar;
use husky_vfs::VfsJar;
use husky_word::WordJar;

#[salsa::db(
    EntityPathJar,
    VfsJar,
    WordJar,
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
    ValidTermJar,
    ValidTypeJar,
    TermJar
)]
#[derive(Default)]
pub(crate) struct TermTestsDb {
    storage: salsa::Storage<TermTestsDb>,
}

impl salsa::Database for TermTestsDb {}
