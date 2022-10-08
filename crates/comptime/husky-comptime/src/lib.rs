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
pub use husky_entity_route::{EntityRoute, InternEntityRoute};
pub use husky_entity_semantics::EntityDefnQueryGroup;
pub use husky_entity_syntax::{EntitySyntaxQueryGroup, EntitySyntaxSalsaQueryGroup};
pub use husky_file::{AllocateUniqueFile, FileQueryGroup, FileSalsaQuery, LiveFiles};
pub use husky_fmt::FmtQuery;
pub use husky_hover_contents::HuskyHoverContentsQuery;
pub use husky_infer_entity_route::*;
pub use husky_infer_qualified_ty::*;
pub use husky_linkage_table::ResolveLinkage;
pub use husky_package_semantics::PackageQueryGroup;
pub use husky_rust_code_gen::RustCodeGenQueryGroup;
pub use husky_token::TokenQueryGroup;
pub use husky_token::TokenSalsaQueryGroup;
pub use husky_word::InternWord;
pub use infer_contract::*;
pub use infer_decl::*;
pub use infer_total::*;
pub use ops::ComptimeOps;
pub use query::*;

use husky_check_utils::*;
use husky_entity_kind::TyKind;
use husky_entity_route::EntityRoutePtr;
use husky_entity_semantics::EntityRouteStore;
use husky_file::FilePtr;
use husky_linkage_table::LinkageTable;
use husky_print_utils::*;
use husky_vm::{__Register, __RegisterDataKind, __VirtualEnum, __VIRTUAL_ENUM_VTABLE};
use husky_word::RootBuiltinIdentifier;
use indexmap::IndexMap;
use std::{fmt, path::PathBuf, sync::Arc};
use sync_utils::ASafeRwLock;

#[salsa::database(
    husky_file::FileQueryStorage,
    husky_token::TokenQueryGroupStorage,
    husky_entity_syntax::ScopeQueryGroupStorage,
    husky_text::TextQueryGroupStorage,
    husky_ast::AstQueryGroupStorage,
    husky_fmt::FormatQueryGroupStorage,
    infer_decl::DeclQueryGroupStorage,
    husky_infer_entity_route::InferEntityRouteQueryGroupStorage,
    infer_contract::InferContractQueryGroupStorage,
    husky_infer_qualified_ty::InferQualifiedTyQueryGroupStorage,
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
    entity_route_interner: Arc<husky_entity_route::EntityRouteInterner>,
    live_docs: ASafeRwLock<IndexMap<FilePtr, ASafeRwLock<String>>>,
    linkage_table: LinkageTable,
    entity_route_store: EntityRouteStore,
    config: ComptimeConfig,
}

impl HuskyComptime {
    pub fn new(config: ComptimeConfig) -> Self {
        let mut comptime = Self {
            storage: Default::default(),
            file_interner: Arc::new(husky_file::new_file_interner()),
            word_interner: Arc::new(husky_word::new_word_interner()),
            live_docs: Default::default(),
            linkage_table: LinkageTable::new(config.linkage_table.clone()),
            entity_route_store: Default::default(),
            config,
            entity_route_interner: Arc::new(husky_entity_route::new_entity_route_interner()),
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
