use std::panic::RefUnwindSafe;

#[salsa::db(
    // comptime
    husky_ki_repr::jar::KiReprJar,
    husky_rust_transpilation::jar::RustTranspilationJar,
    // devtime
    husky_trace::jar::TraceJar,
    // fs
    husky_corgi_config::CorgiConfigJar,
    husky_manifest::ManifestJar,
    husky_vfs::VfsJar,
    // hir
    husky_hir_eager_expr::jar::HirEagerExprJar,
    husky_hir_lazy_expr::jar::HirLazyExprJar,
    husky_hir_expr::jar::HirExprJar,
    husky_hir_decl::jar::HirDeclJar,
    husky_hir_defn::jar::HirDefnJar,
    // ide
    husky_token_info::db::TokenInfoJar,
    // kernel
    husky_coword::CowordJar,
    husky_entity_path::jar::EntityPathJar,
    husky_term_prelude::TermPreludeJar,
    husky_dec_term::jar::DecTermJar,
    husky_dec_ty::DeclarativeTypeJar,
    husky_dec_signature::DecSignatureJar,
    husky_eth_term::EthTermJar,
    husky_eth_signature::EtherealSignatureJar,
    husky_fly_term::FlyTermJar,
    husky_hir_ty::db::HirTypeJar,
    // lex
    husky_token_data::jar::TokenDataJar,
    husky_token::db::TokenJar,
    husky_toml_token::jar::TomlTokenJar,
    husky_text::jar::TextJar,
    // linkage
    husky_javelin::jar::JavelinJar,
    husky_linkage::jar::LinkageJar,
    // semantics
    husky_sem_expr::SemExprJar,
    husky_sem_place_contract::jar::SemaPlaceContractJar,
    // syntax
    husky_ast::jar::AstJar,
    husky_toml_ast::TomlAstJar,
    husky_corgi_config_ast::CorgiConfigAstJar,
    husky_manifest_ast::ManifestAstJar,
    husky_entity_tree::EntityTreeJar,
    husky_syn_expr::jar::SynExprJar,
    husky_syn_decl::SynDeclJar,
    husky_syn_defn::SynDefnJar,
    // val
    husky_ki::jar::KiJar
)]
pub struct DevComptimeDb;

// ad hoc: is this correct?
impl RefUnwindSafe for DevComptimeDb {}
