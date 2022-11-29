mod codegen;
mod config;
mod ide;
mod necessary;
mod ops;
mod query;
pub mod utils;

pub use config::*;
pub use husky_ast::AstDb;
use husky_ast::AstJar;
pub use husky_completion::HuskyCompletionQuery;
pub use husky_diagnostics::DiagnosticsDb;
use husky_diagnostics::DiagnosticsJar;
use husky_entity_path::EntityPathJar;
pub use husky_entity_tree::EntityTreeDb;
use husky_entity_tree::EntityTreeJar;
pub use husky_fmt::SyntaxFormatDb;
use husky_fmt::SyntaxFormatJar;
pub use husky_hover::HoverDb;
pub use husky_identifier::IdentifierDb;
use husky_layout::LayoutJar;
pub use husky_linkage_table::ResolveLinkage;
pub use husky_rust_code_gen::RustTranspileDb;
use husky_rust_code_gen::RustTranspileJar;
use husky_term::TermJar;
use husky_text::TextJar;
pub use husky_token_sheet::TokenTextDb;
use husky_token_sheet::TokenTextJar;
use husky_vfs::VfsJar;
use husky_word::WordJar;
pub use ops::ComptimeOps;
pub use query::*;

use husky_check_utils::*;
use husky_linkage_table::LinkageTable;
use husky_source_path::{SourcePath, SourcePathJar};
use husky_vm::{__Register, __RegisterDataKind, __VirtualEnum, __VIRTUAL_ENUM_VTABLE};
use indexmap::IndexMap;
use std::{fmt, path::PathBuf, sync::Arc};
use sync_utils::ASafeRwLock;

#[salsa::db(
    TokenTextJar,
    EntityTreeJar,
    TextJar,
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
pub struct HuskyComptime {
    storage: salsa::Storage<HuskyComptime>,
    // entity_route_interner: Arc<husky_term::EntityRouteInterner>,
    live_docs: ASafeRwLock<IndexMap<SourcePath, ASafeRwLock<String>>>,
    linkage_table: LinkageTable,
    // entity_route_store: EntityRouteStore,
    config: ComptimeConfig,
}

impl HuskyComptime {
    pub fn new(config: ComptimeConfig) -> Self {
        todo!()
        // let mut comptime = Self {
        //     storage: Default::default(),
        //     live_docs: Default::default(),
        //     linkage_table: LinkageTable::new(config.linkage_table.clone()),
        //     // entity_route_store: Default::default(),
        //     config,
        //     // entity_route_interner: Default::default(),
        // };
        // let target_entrance = comptime.intern_path(comptime.config.package_dir.join("main.hsy"));
        // comptime.set_opt_target_entrance(Some(target_entrance));
        // comptime
    }

    pub fn new_default(package_dir: PathBuf) -> Self {
        Self::new(ComptimeConfig {
            package_dir,
            linkage_table: Default::default(),
        })
    }
}
