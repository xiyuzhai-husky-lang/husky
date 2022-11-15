mod codegen;
mod config;
mod ide;
mod necessary;
mod ops;
mod query;
pub mod utils;

pub use config::*;
pub use husky_ast::{AstQueryGroup, AstSalsaQueryGroup};
pub use husky_completion::HuskyCompletionQuery;
pub use husky_diagnostics::DiagnosticSalsaQuery;
pub use husky_entity_semantics::EntityDefnQueryGroup;
pub use husky_entity_syntax::{EntitySyntaxQueryGroup, EntitySyntaxSalsaQueryGroup};
pub use husky_file::{FileQueryGroup, FileSalsaQuery, InternFile, LiveFiles};
pub use husky_fmt::FmtQuery;
pub use husky_hover::HoverDb;
pub use husky_linkage_table::ResolveLinkage;
pub use husky_package_semantics::PackageQueryGroup;
pub use husky_rust_code_gen::RustCodeGenQueryGroup;
pub use husky_token_syntax::Tokenize;
pub use husky_token_syntax::TokenizedTextQueryGroup;
pub use husky_word::InternWord;
pub use ops::ComptimeOps;
pub use query::*;

use husky_check_utils::*;
use husky_entity_semantics::EntityRouteStore;
use husky_file::FileItd;
use husky_linkage_table::LinkageTable;
use husky_vm::{__Register, __RegisterDataKind, __VirtualEnum, __VIRTUAL_ENUM_VTABLE};
use indexmap::IndexMap;
use std::{fmt, path::PathBuf, sync::Arc};
use sync_utils::ASafeRwLock;

#[salsa::database(
    husky_file::FileQueryStorage,
    husky_token_syntax::TokenQueryGroupStorage,
    husky_entity_syntax::ScopeQueryGroupStorage,
    husky_text::TextQueryGroupStorage,
    husky_ast::AstQueryGroupStorage,
    husky_fmt::FormatQueryGroupStorage,
    husky_entity_semantics::EntityQueryGroupStorage,
    husky_package_semantics::PackageQueryGroupStorage,
    husky_diagnostics::DiagnosticSalsaQueryGroupStorage,
    husky_rust_code_gen::RustGenQueryStorage,
    husky_layout::HuskyLayoutQueryGroupStorage
)]
pub struct HuskyComptime {
    storage: salsa::Storage<HuskyComptime>,
    file_interner: Arc<husky_file::FileInterner>,
    word_interner: Arc<husky_word::WordInterner>,
    // entity_route_interner: Arc<husky_term::EntityRouteInterner>,
    live_docs: ASafeRwLock<IndexMap<FileItd, ASafeRwLock<String>>>,
    linkage_table: LinkageTable,
    entity_route_store: EntityRouteStore,
    config: ComptimeConfig,
}

impl HuskyComptime {
    pub fn new(config: ComptimeConfig) -> Self {
        let mut comptime = Self {
            storage: Default::default(),
            file_interner: Default::default(),
            word_interner: Default::default(),
            live_docs: Default::default(),
            linkage_table: LinkageTable::new(config.linkage_table.clone()),
            entity_route_store: Default::default(),
            config,
            // entity_route_interner: Default::default(),
        };
        let target_entrance = comptime.intern_file(comptime.config.package_dir.join("main.hsy"));
        comptime.set_opt_target_entrance(Some(target_entrance));
        comptime
    }

    pub fn new_default(
        package_dir: PathBuf,
        __root_defn: fn(
            ident: husky_word::RootBuiltinIdentifier,
        ) -> &'static husky_static_defn::EntityStaticDefn,
    ) -> Self {
        Self::new(ComptimeConfig {
            package_dir,
            __resolve_root_defn: __root_defn,
            linkage_table: Default::default(),
        })
    }
}
