use husky_ast::AstJar;
use husky_diagnostics::DiagnosticsJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::EntityTreeJar;
use husky_fmt::SyntaxFormatJar;
use husky_layout::LayoutJar;
use husky_package_path::PackagePathJar;
use husky_rust_code_gen::RustTranspileJar;
use husky_term::TermJar;
use husky_token::TokenJar;
use husky_toolchain::ToolchainJar;
use husky_vfs::{HasVfsConfig, VfsConfig, VfsConfigImpl, VfsJar};
use husky_word::WordJar;

#[salsa::db(
    TokenJar,
    PackagePathJar,
    EntityTreeJar,
    ToolchainJar,
    AstJar,
    WordJar,
    EntityPathJar,
    TermJar,
    VfsJar,
    SyntaxFormatJar,
    DiagnosticsJar,
    RustTranspileJar,
    LayoutJar
)]
#[derive(Default)]
pub struct AnalyzerDB {
    storage: salsa::Storage<AnalyzerDB>,
    source_path_config: VfsConfigImpl,
}

impl salsa::Database for AnalyzerDB {}

impl HasVfsConfig for AnalyzerDB {
    fn vfs_config(&self) -> &VfsConfig {
        &self.source_path_config
    }
}

impl salsa::ParallelDatabase for AnalyzerDB {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(AnalyzerDB {
            storage: self.storage.snapshot(),
            source_path_config: self.source_path_config.clone(),
        })
    }
}

// ad hoc
impl std::panic::RefUnwindSafe for AnalyzerDB {}
// ad hoc
impl std::panic::UnwindSafe for AnalyzerDB {}
