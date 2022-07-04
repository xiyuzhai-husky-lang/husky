mod impl_diagnostics;
mod impl_load;
mod impl_necessary;
pub mod utils;

pub use husky_ast::{AstQueryGroup, AstSalsaQueryGroup};
pub use husky_diagnostics::DiagnosticQuery;
pub use husky_entity_route::{EntityRoute, InternEntityRoute};
pub use husky_entity_semantics::EntityDefnQueryGroup;
pub use husky_entity_syntax::{EntitySyntaxQueryGroup, EntitySyntaxSalsaQueryGroup};
pub use husky_file::{AllocateUniqueFile, FileQueryGroup, FileSalsaQuery, LiveFiles};
pub use husky_fmt::FmtQuery;
pub use husky_infer_entity_route::*;
pub use husky_package_semantics::PackageQueryGroup;
pub use husky_rust_code_gen::RustCodeGenQueryGroup;
pub use husky_token::TokenQueryGroup;
pub use husky_token::TokenSalsaQueryGroup;
use indexmap::IndexMap;
pub use infer_contract::*;
pub use infer_decl::*;
pub use infer_qualifier::*;
pub use infer_total::*;
pub use word::InternWord;

use check_utils::*;
use husky_entity_route::EntityRoutePtr;
use husky_entity_semantics::EntityRouteStore;
use husky_file::FilePtr;
use husky_linkage_table::LinkageTable;
use print_utils::*;
use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, Mutex},
};
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
    infer_qualifier::InferQualifiedTyQueryGroupStorage,
    husky_entity_semantics::EntityQueryGroupStorage,
    husky_package_semantics::PackageQueryGroupStorage,
    husky_diagnostics::DiagnosticQueryGroupStorage,
    husky_rust_code_gen::RustGenQueryStorage
)]
pub struct HuskyCompileTime {
    storage: salsa::Storage<HuskyCompileTime>,
    static_ty_cache: Arc<Mutex<HashMap<std::any::TypeId, EntityRoutePtr>>>,
    file_interner: Arc<husky_file::FileInternerSingletonKeeper>,
    word_interner: Arc<word::WordInternerSingletonKeeper>,
    scope_interner: Arc<husky_entity_route::EntityRouteInternerSingletonKeeper>,
    live_docs: ASafeRwLock<IndexMap<FilePtr, ASafeRwLock<String>>>,
    linkage_table: LinkageTable,
    entity_route_store: EntityRouteStore,
    opt_main: Option<FilePtr>,
    __root_defn_resolver: fn(ident: word::RootIdentifier) -> &'static static_defn::EntityStaticDefn,
}

impl HuskyCompileTime {
    pub fn new(
        __root_defn_resolver: fn(
            ident: word::RootIdentifier,
        ) -> &'static static_defn::EntityStaticDefn,
    ) -> Self {
        let live_docs = Default::default();
        let scope_interner = husky_entity_route::new_entity_route_interner();
        let entity_route_store = Default::default();
        let husky_linkage_table = Default::default();
        Self {
            storage: Default::default(),
            file_interner: husky_file::new_file_interner(),
            word_interner: word::new_word_interner(),
            scope_interner,
            live_docs,
            linkage_table: husky_linkage_table,
            entity_route_store,
            opt_main: None,
            __root_defn_resolver,
            static_ty_cache: Default::default(),
        }
    }

    pub fn main_file(&self) -> FilePtr {
        self.opt_main.unwrap()
    }
}

pub trait AskCompileTime {
    fn compile_time(&self) -> &HuskyCompileTime;
}
