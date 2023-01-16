use std::sync::Arc;

use dashmap::DashMap;
use husky_ast::AstJar;
use husky_decl::DeclJar;
use husky_defn::DefnJar;
use husky_diagnostics::DiagnosticsJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::EntityTreeJar;
use husky_expr::ExprJar;
use husky_folding_range::FoldingRangeJar;
use husky_hover::HoverJar;
use husky_layout::LayoutJar;
use husky_manifest::ManifestJar;
use husky_rust_code_gen::RustTranspileJar;
use husky_semantic_token::SemanticTokenJar;
use husky_syntax_fmt::SyntaxFormatJar;
use husky_term::TermJar;
use husky_token::TokenJar;
use husky_token_info::TokenInfoJar;
use husky_vfs::*;
use husky_word::WordJar;

#[salsa::db(
    TokenJar,
    VfsJar,
    EntityTreeJar,
    AstJar,
    WordJar,
    EntityPathJar,
    TermJar,
    SyntaxFormatJar,
    DiagnosticsJar,
    RustTranspileJar,
    LayoutJar,
    ManifestJar,
    // syntax
    ExprJar,
    DeclJar,
    DefnJar,
    // infer
    TokenInfoJar,
    // ide
    FoldingRangeJar,
    SemanticTokenJar,
    HoverJar,
)]
#[derive(Default)]
pub struct AnalyzerDB {
    storage: salsa::Storage<AnalyzerDB>,
    semantic_tokens_ext_cache: Arc<DashMap<lsp_types::Url, Vec<lsp_types::SemanticToken>>>,
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
    pub(crate) fn cache_semantic_tokens_ext(
        &self,
        uri: lsp_types::Url,
        semantic_tokens_ext: &[lsp_types::SemanticToken],
    ) {
        self.semantic_tokens_ext_cache
            .insert(uri, semantic_tokens_ext.to_vec());
    }
}

// ad hoc
impl std::panic::RefUnwindSafe for AnalyzerDB {}
// ad hoc
impl std::panic::UnwindSafe for AnalyzerDB {}
