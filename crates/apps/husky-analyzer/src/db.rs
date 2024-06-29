use std::sync::Arc;

use dashmap::DashMap;
use salsa::snapshot::SnapshotClone;

#[salsa::db(
    // fs
    husky_vfs::jar::VfsJar,
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
    husky_place::jar::PlaceJar,
    // lex
    husky_text::jar::TextJar,
    husky_token_data::jar::TokenDataJar,
    husky_token::TokenJar,
    husky_toml_token::jar::TomlTokenJar,
    // syntax
    husky_ast::jar::AstJar,
    husky_toml_ast::TomlAstJar,
    husky_corgi_config_ast::CorgiConfigAstJar,
    husky_manifest_ast::jar::ManifestAstJar,
    husky_entity_tree::jar::EntityTreeJar,
    husky_syn_expr::jar::SynExprJar,
    husky_syn_decl::jar::SynDeclJar,
    husky_syn_defn::jar::SynDefnJar,
    // semantics
    husky_sem_expr::jar::SemExprJar,
    husky_sem_place_contract::jar::SemPlaceContractJar,
    husky_corgi_config::jar::CorgiConfigJar,
    husky_manifest::jar::ManifestJar,
    // ide
    husky_token_info::TokenInfoJar,
    husky_folding_range::jar::FoldingRangeJar,
    husky_semantic_token::SemanticTokenJar,
    husky_hover::HoverJar,
    husky_ide_fmt::jar::IdeFmtJar,
    husky_inlay_hints::jar::InlayHintsJar,
    husky_diagnostics::DiagnosticsJar,
    husky_code_lens::jar::CodeLensJar,
)]
pub struct AnalyzerDb;

#[derive(Default)]
pub struct AnalyzerDB {
    db: AnalyzerDb,
    semantic_tokens_ext_cache: Arc<DashMap<lsp_types::Url, lsp_types::SemanticTokens>>,
}

impl SnapshotClone for AnalyzerDB {
    fn snapshot_clone(&self) -> Self {
        Self {
            db: self.db.snapshot_clone(),
            semantic_tokens_ext_cache: self.semantic_tokens_ext_cache.clone(),
        }
    }
}

pub type AnalyzerDBSnapshot = ::salsa::snapshot::Snapshot<AnalyzerDB>;

impl std::ops::Deref for AnalyzerDB {
    type Target = ::salsa::Db;

    fn deref(&self) -> &Self::Target {
        &*self.db
    }
}

impl std::ops::DerefMut for AnalyzerDB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.db
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
