pub(crate) use husky_ast::test_utils::*;

use crate::*;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_coword::CowordJar;
use husky_dec_signature::DecSignatureJar;
use husky_dec_ty::DeclarativeTypeJar;
use husky_entity_path::{MajorItemPath, TypePath};
use husky_entity_tree::{EntityTreeDb, EntityTreeJar};
use husky_eth_signature::EtherealSignatureJar;
use husky_eth_term::EthTermJar;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_print_utils::p;
use husky_syn_decl::{SynDeclDb, SynDeclJar};
use husky_syn_expr::SynExprJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::jar::TomlTokenJar;
use husky_vfs::*;

#[salsa::db(
    CowordJar,
    VfsJar,
    husky_entity_path::jar::EntityPathJar,
    husky_token_data::jar::TokenDataJar,
    husky_text::jar::TextJar,
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
    SynDefnJar,
    SynDeclJar,
    TermPreludeJar,
    husky_dec_term::jar::DecTermJar,
    DecSignatureJar,
    husky_dec_ty::jar::DeclarativeTypeJar,
    EthTermJar,
    EtherealSignatureJar,
    FlyTermJar,
    SemaExprJar
)]
#[derive(Default)]
pub(crate) struct DB;

fn decl_sem_expr_regions(db: &::salsa::Db, module_path: ModulePath) -> Vec<SemaExprRegion> {
    use husky_syn_decl::HasSynDeclSheet;

    module_path
        .syn_decl_sheet(db)
        .decls(db)
        .iter()
        .copied()
        .filter_map(|(_, decl)| Some(db.sem_expr_region(decl.syn_expr_region(db)?)))
        .collect()
}

#[test]
fn decl_sem_expr_regions_works() {
    DB::ast_expect_test_debug_with_db(
        decl_sem_expr_regions,
        &AstTestConfig::new(
            "decl_sem_expr_regions",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SEMANTICS,
        ),
    )
}

fn defn_sem_expr_regions(db: &::salsa::Db, module_path: ModulePath) -> Vec<SemaExprRegion> {
    module_item_syn_defns(db, module_path)
        .iter()
        .copied()
        .filter_map(|(_, defn)| Some(db.sem_expr_region(defn?.syn_expr_region)))
        .collect()
}

#[test]
fn defn_sem_expr_regions_works() {
    DB::ast_expect_test_debug_with_db(
        defn_sem_expr_regions,
        &AstTestConfig::new(
            "defn_sem_expr_regions",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SEMANTICS,
        ),
    )
}
