mod impl_diagnostics;
mod impl_load;
mod impl_necessary;
#[cfg(test)]
mod tests;
pub mod utils;

pub use ast::{AstQueryGroup, AstSalsaQueryGroup};
pub use diagnostic::DiagnosticQuery;
pub use entity_route::{AllocateUniqueScope, EntityRoute};
pub use entity_syntax::{EntitySyntaxQueryGroup, EntitySyntaxSalsaQueryGroup};
pub use feature_gen::{AllocateUniqueFeature, FeatureGenQueryGroup, FeatureGenQueryGroupStorage};
pub use file::{AllocateUniqueFile, FileQueryGroup, FileSalsaQuery, LiveFiles};
pub use husky_fmt::FmtQuery;
use indexmap::IndexMap;
pub use infer_contract::*;
pub use infer_decl::*;
pub use infer_entity_route::*;
pub use infer_qualifier::*;
pub use infer_total::*;
pub use instruction_gen::InstructionGenQueryGroup;
pub use pack_semantics::PackageQueryGroup;
pub use rust_gen::RustGenQueryGroup;
pub use semantics_entity::EntityDefnQueryGroup;
pub use token::TokenQueryGroup;
pub use token::TokenSalsaQueryGroup;
pub use visualizer_gen::VisualizerQueryGroup;
pub use word::InternWord;

use check_utils::*;
use entity_route::EntityRoutePtr;
use file::FilePtr;
use linkage_table::LinkageSourceTable;
use print_utils::*;
use semantics_entity::EntityRouteStore;
use std::{fmt, sync::Arc};
use sync_utils::ARwLock;

#[salsa::database(
    file::FileQueryStorage,
    token::TokenQueryGroupStorage,
    entity_syntax::ScopeQueryGroupStorage,
    text::TextQueryGroupStorage,
    ast::AstQueryGroupStorage,
    husky_fmt::FormatQueryGroupStorage,
    infer_decl::DeclQueryGroupStorage,
    infer_entity_route::InferEntityRouteQueryGroupStorage,
    infer_contract::InferContractQueryGroupStorage,
    infer_qualifier::InferQualifiedTyQueryGroupStorage,
    semantics_entity::EntityQueryGroupStorage,
    pack_semantics::PackageQueryGroupStorage,
    feature_gen::FeatureGenQueryGroupStorage,
    diagnostic::DiagnosticQueryGroupStorage,
    instruction_gen::InstructionGenQueryGroupStorage,
    rust_gen::RustGenQueryStorage,
    visualizer_gen::VisualizerQueryGroupStorage
)]
pub struct HuskyCompileTime {
    storage: salsa::Storage<HuskyCompileTime>,
    file_unique_allocator: file::FileInterner,
    word_unique_allocator: word::WordAllocator,
    scope_unique_allocator: entity_route::EntityRouteInterner,
    live_docs: ARwLock<IndexMap<FilePtr, ARwLock<String>>>,
    features: feature_gen::FeatureInterner,
    linkage_table: LinkageSourceTable,
    entity_route_store: EntityRouteStore,
    opt_main: Option<FilePtr>,
}

impl HuskyCompileTime {
    pub fn main_file(&self) -> FilePtr {
        self.opt_main.unwrap()
    }
}

pub trait AskCompileTime {
    fn compile_time(&self) -> &HuskyCompileTime;
}
