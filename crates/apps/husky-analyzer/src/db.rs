use std::sync::Arc;

use dashmap::DashMap;
use husky_ast::AstJar;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_decl::DeclJar;
use husky_declarative_term::DeclarativeTermJar;
use husky_declarative_ty::DeclarativeTypeJar;
use husky_decr::DecrJar;
use husky_defn::DefnJar;
use husky_diagnostics::DiagnosticsJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::EntityTreeJar;
use husky_ethereal_term::EtherealTermJar;
use husky_expr::ExprJar;
use husky_expr_ty::ExprTypeJar;
use husky_fluffy_term::FluffyTermJar;
use husky_folding_range::FoldingRangeJar;
use husky_hover::HoverJar;
use husky_layout::LayoutJar;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_rust_code_gen::RustTranspileJar;
use husky_semantic_token::SemanticTokenJar;
use husky_signature::DeclarativeSignatureJar;
use husky_syntax_fmt::SyntaxFormatJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_token_info::TokenInfoJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;
use husky_vfs::*;
use husky_word::WordJar;

#[salsa::db(
    TokenJar,
    VfsJar,
    EntityTreeJar,
    AstJar,
    WordJar,
    EntityPathJar,
    SyntaxFormatJar,
    DiagnosticsJar,
    RustTranspileJar,
    LayoutJar,
    TomlTokenJar,TomlAstJar,ManifestAstJar,
    CorgiConfigJar,CorgiConfigAstJar, ManifestJar,
    // kernel
    TermPreludeJar,
    DeclarativeTermJar,
    DeclarativeTypeJar,
    EtherealTermJar,
    // syntax
    ExprJar,
    DeclJar,
    DecrJar,
    DefnJar,
    // infer
    TokenInfoJar,
    DeclarativeSignatureJar,
    FluffyTermJar,
    ExprTypeJar,
    // ide
    FoldingRangeJar,
    SemanticTokenJar,
    HoverJar,
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
