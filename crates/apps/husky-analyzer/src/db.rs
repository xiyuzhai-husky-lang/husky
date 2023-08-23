use std::sync::Arc;

use dashmap::DashMap;

#[salsa::db(
    // fs
    husky_vfs::VfsJar,
    // kernel
    husky_coword::CowordJar,
    husky_entity_path::EntityPathJar,
    husky_term_prelude::TermPreludeJar,
    husky_declarative_term::DeclarativeTermJar,
    husky_declarative_ty::DeclarativeTypeJar,
    husky_declarative_signature::DeclarativeSignatureJar,
    husky_ethereal_term::EtherealTermJar,
    husky_ethereal_signature::EtherealSignatureJar,
    husky_fluffy_term::FluffyTermJar,
    // lex
    husky_token::TokenJar,
    husky_toml_token::TomlTokenJar,
    // syntax
    husky_ast::AstJar,
    husky_toml_ast::TomlAstJar,
    husky_corgi_config_ast::CorgiConfigAstJar,
    husky_manifest_ast::ManifestAstJar,
    husky_entity_syn_tree::EntitySynTreeJar,
    husky_syn_expr::SynExprJar,
    husky_syn_decl::SynDeclJar,
    husky_syn_decr::SynDecrJar,
    husky_syn_defn::SynDefnJar,
    // semantics
    husky_expr_ty::ExprTypeJar,
    husky_corgi_config::CorgiConfigJar,
    husky_manifest::ManifestJar,
    // ide
    husky_token_info::TokenInfoJar,
    husky_folding_range::FoldingRangeJar,
    husky_semantic_token::SemanticTokenJar,
    husky_hover::HoverJar,
    husky_syn_fmt::SyntaxFormatJar,
    husky_diagnostics::DiagnosticsJar,
)]
#[derive(Default)]
pub struct AnalyzerDB {
    storage: salsa::Storage<AnalyzerDB>,
    semantic_tokens_ext_cache: Arc<DashMap<lsp_types::Url, lsp_types::SemanticTokens>>,
}

impl salsa::Database for AnalyzerDB {}

impl salsa::ParallelDatabase for AnalyzerDB {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(AnalyzerDB {
            storage: self.storage.snapshot(),
            semantic_tokens_ext_cache: self.semantic_tokens_ext_cache.clone(),
        })
    }
}

impl AnalyzerDB {
    pub(crate) fn cache_semantic_tokens(
        &self,
        uri: lsp_types::Url,
        semantic_tokens: lsp_types::SemanticTokens,
    ) {
        self.semantic_tokens_ext_cache.insert(uri, semantic_tokens);
    }

    pub(crate) fn cached_semantic_tokens_entry(
        &self,
        uri: lsp_types::Url,
    ) -> dashmap::mapref::entry::Entry<lsp_types::Url, lsp_types::SemanticTokens> {
        self.semantic_tokens_ext_cache.entry(uri)
    }
}

// ad hoc
impl std::panic::RefUnwindSafe for AnalyzerDB {}
// ad hoc
impl std::panic::UnwindSafe for AnalyzerDB {}
