use crate::TermJar;
use husky_ast::AstJar;
use husky_decl::DeclJar;
use husky_entity_path::{EntityPathJar, ModuleItemPath, TypePath};
use husky_entity_tree::{EntityTreeDb, EntityTreeJar};
use husky_expr::ExprJar;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_raw_term::RawTermJar;
use husky_raw_ty::RawTypeJar;
use husky_signature::SignatureJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;
use husky_vfs::VfsJar;
use husky_word::WordJar;

#[salsa::db(
    EntityPathJar,
    VfsJar,
    WordJar,
    TokenJar,
    AstJar,
    EntityTreeJar,
    TomlTokenJar,
    TomlAstJar,
    ManifestAstJar,
    ManifestJar,
    ExprJar,
    DeclJar,
    TermPreludeJar,
    RawTermJar,
    SignatureJar,
    RawTypeJar,
    TermJar
)]
#[derive(Default)]
pub(crate) struct TermTestsDb {
    storage: salsa::Storage<TermTestsDb>,
}

impl salsa::Database for TermTestsDb {}
