use std::panic::RefUnwindSafe;

#[salsa::db(
    // comptime
    husky_ki_repr::jar::KiReprJar,
    husky_rust_transpilation::jar::RustTranspilationJar,
    // devtime
    husky_trace::jar::TraceJar,
    // fs
    husky_corgi_config::jar::CorgiConfigJar,
    husky_manifest::jar::ManifestJar,
    husky_vfs::jar::VfsJar,
    // hir
    husky_hir_prelude::jar::HirPreludeJar,
    husky_hir_eager_expr::jar::HirEagerExprJar,
    husky_hir_lazy_expr::jar::HirLazyExprJar,
    husky_hir_expr::jar::HirExprJar,
    husky_hir_decl::jar::HirDeclJar,
    husky_hir_defn::jar::HirDefnJar,
    // ide
    husky_token_info::jar::TokenInfoJar,
    // kernel
    husky_coword::jar::CowordJar,
    husky_entity_path::jar::EntityPathJar,
    husky_term_prelude::jar::TermPreludeJar,
    husky_dec_term::jar::DecTermJar,
    husky_dec_ty::DecTypeJar,
    husky_dec_signature::jar::DecSignatureJar,
    husky_eth_term::jar::EthTermJar,
    husky_eth_signature::jar::EthSignatureJar,
    husky_fly_term::jar::FlyTermJar,
    husky_hir_ty::jar::HirTypeJar,
    husky_place::jar::PlaceJar,
    // lex
    husky_token_data::jar::TokenDataJar,
    husky_token::jar::TokenJar,
    husky_toml_token::jar::TomlTokenJar,
    husky_text::jar::TextJar,
    // linkage
    husky_javelin::jar::JavelinJar,
    husky_linkage::jar::LinkageJar,
    // semantics
    husky_sem_expr::SemExprJar,
    husky_sem_place_contract::jar::SemPlaceContractJar,
    husky_sem_item_path_deps::jar::SemItemPathDepsJar,
    husky_sem_static_mut_deps::jar::SemStaticMutDepsJar,
    husky_sem_var_deps::jar::SemVarDepsJar,
    // syntax
    husky_ast::jar::AstJar,
    husky_toml_ast::TomlAstJar,
    husky_corgi_config_ast::CorgiConfigAstJar,
    husky_manifest_ast::jar::ManifestAstJar,
    husky_entity_tree::jar::EntityTreeJar,
    husky_syn_expr::jar::SynExprJar,
    husky_syn_decl::jar::SynDeclJar,
    husky_syn_defn::jar::SynDefnJar,
    // val
    husky_ki::jar::KiJar
)]
pub struct DevComptimeDb;

// ad hoc: is this correct?
impl RefUnwindSafe for DevComptimeDb {}
