mod codegen;
mod config;
mod ide;
mod necessary;
mod ops;
mod query;
pub mod utils;

pub use config::*;
pub use husky_ast::AstDb;
pub use husky_completion::HuskyCompletionQuery;
pub use husky_diagnostics::DiagnosticsDb;
pub use husky_entity_tree::EntityTreeDb;
pub use husky_fmt::SyntaxFormatDb;
pub use husky_hover::HoverDb;
pub use husky_linkage_table::ResolveLinkage;
use husky_package_path::PackagePathJar;
pub use husky_rust_code_gen::RustTranspileDb;
pub use husky_token::TokenDb;
pub use husky_word::WordDb;
pub use query::*;

use husky_ast::AstJar;
use husky_check_utils::*;
use husky_diagnostics::DiagnosticsJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::EntityTreeJar;
use husky_fmt::SyntaxFormatJar;
use husky_layout::LayoutJar;
use husky_linkage_table::LinkageTable;
use husky_rust_code_gen::RustTranspileJar;
use husky_source_path::{
    HasSourcePathConfig, SourcePath, SourcePathConfig, SourcePathConfigImpl, SourcePathJar,
};
use husky_term::TermJar;
use husky_token::TokenJar;
use husky_toolchain::ToolchainJar;
use husky_vfs::VfsJar;
use husky_vm::{__Register, __RegisterDataKind, __VirtualEnum, __VIRTUAL_ENUM_VTABLE};
use husky_word::WordJar;
use indexmap::IndexMap;
use std::{fmt, path::PathBuf, sync::Arc};
use sync_utils::ASafeRwLock;

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

impl HasSourcePathConfig for AnalysisHost {
    fn source_path_config(&self) -> &SourcePathConfig {
        &self.source_path_config
    }
}

// impl HuskyComptime {
//     pub fn new(config: ComptimeConfig) -> Self {
//         todo!()
//         // let mut comptime = Self {
//         //     storage: Default::default(),
//         //     live_docs: Default::default(),
//         //     linkage_table: LinkageTable::new(config.linkage_table.clone()),
//         //     // entity_route_store: Default::default(),
//         //     config,
//         //     // entity_route_interner: Default::default(),
//         // };
//         // let target_entrance = comptime.intern_path(comptime.config.package_dir.join("main.hsy"));
//         // comptime.set_opt_target_entrance(Some(target_entrance));
//         // comptime
//     }

//     pub fn new_default(package_dir: PathBuf) -> Self {
//         Self::new(ComptimeConfig {
//             package_dir,
//             linkage_table: Default::default(),
//         })
//     }
// }
