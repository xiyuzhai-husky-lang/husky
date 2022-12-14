use husky_ast::AstJar;
use husky_diagnostics::DiagnosticsJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::EntityTreeJar;
use husky_fmt::SyntaxFormatJar;
use husky_layout::LayoutJar;
use husky_package_path::PackagePathJar;
use husky_rust_code_gen::RustTranspileJar;
use husky_source_path::{
    HasSourcePathConfig, SourcePathConfig, SourcePathConfigImpl, SourcePathJar,
};
use husky_term::TermJar;
use husky_token::TokenJar;
use husky_toolchain::ToolchainJar;
use husky_vfs::VfsJar;
use husky_word::WordJar;

#[salsa::db(
    TokenJar,
    PackagePathJar,
    EntityTreeJar,
    ToolchainJar,
    AstJar,
    WordJar,
    SourcePathJar,
    EntityPathJar,
    TermJar,
    VfsJar,
    SyntaxFormatJar,
    DiagnosticsJar,
    RustTranspileJar,
    LayoutJar
)]
#[derive(Default)]
pub struct AnalysisHost {
    storage: salsa::Storage<AnalysisHost>,
    source_path_config: SourcePathConfigImpl,
}

impl salsa::Database for AnalysisHost {}

impl HasSourcePathConfig for AnalysisHost {
    fn source_path_config(&self) -> &SourcePathConfig {
        &self.source_path_config
    }
}

impl salsa::ParallelDatabase for AnalysisHost {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(AnalysisHost {
            storage: self.storage.snapshot(),
            source_path_config: self.source_path_config.clone(),
        })
    }
}
