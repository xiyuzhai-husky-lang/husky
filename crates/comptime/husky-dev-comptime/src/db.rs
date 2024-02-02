use std::panic::RefUnwindSafe;

#[salsa::db(
    // comptime
    husky_val_repr::jar::ValReprJar,
    husky_rust_transpilation::db::RustTranspilationJar,
    // devtime
    husky_trace::db::TraceJar,
    // fs
    husky_vfs::VfsJar,
    // hir
    husky_hir_eager_expr::db::HirEagerExprJar,
    husky_hir_lazy_expr::db::HirLazyExprJar,
    husky_hir_expr::db::HirExprJar,
    husky_hir_decl::db::HirDeclJar,
    husky_hir_defn::db::HirDefnJar,
    // ide
    husky_token_info::db::TokenInfoJar,
    // kernel
    husky_coword::CowordJar,
    husky_entity_path::jar::EntityPathJar,
    husky_term_prelude::TermPreludeJar,
    husky_dec_term::DecTermJar,
    husky_dec_ty::DeclarativeTypeJar,
    husky_dec_signature::DecSignatureJar,
    husky_eth_term::EthTermJar,
    husky_eth_signature::EtherealSignatureJar,
    husky_fly_term::FlyTermJar,
    husky_hir_ty::db::HirTypeJar,
    // lex
    husky_token_data::db::TokenDataJar,
    husky_token::TokenJar,
    husky_toml_token::TomlTokenJar,
    husky_text::db::TextJar,
    // linkage
    husky_javelin::jar::JavelinJar,
    husky_linkage::jar::LinkageJar,
    // semantics
    husky_sema_expr::SemaExprJar,
    husky_corgi_config::CorgiConfigJar,
    husky_manifest::ManifestJar,
    // syntax
    husky_ast::jar::AstJar,
    husky_toml_ast::TomlAstJar,
    husky_corgi_config_ast::CorgiConfigAstJar,
    husky_manifest_ast::ManifestAstJar,
    husky_entity_tree::EntityTreeJar,
    husky_syn_expr::SynExprJar,
    husky_syn_decl::SynDeclJar,
    husky_syn_defn::SynDefnJar,
    // val
    husky_val::jar::ValJar
)]
pub struct DevComptimeDb;

// ad hoc: is this correct?
impl RefUnwindSafe for DevComptimeDb {}
