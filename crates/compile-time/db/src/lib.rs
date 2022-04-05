mod impl_load;
mod impl_necessary;
#[cfg(test)]
mod tests;

pub use ast::{AstQueryGroup, AstSalsaQueryGroup};
pub use decl::*;
pub use diagnostic::DiagnosticQuery;
pub use entity_route::{AllocateUniqueScope, Route};
pub use entity_route_query::{EntityRouteSalsaQueryGroup, ScopeQueryGroup};
pub use feature::{AllocateUniqueFeature, FeatureQueryGroup, FeatureQueryGroupStorage};
pub use file::{AllocateUniqueFile, FileQueryGroup, FileSalsaQuery, LiveFiles};
pub use husky_fmt::FmtQuery;
pub use infer_contract::*;
pub use infer_total::*;
pub use infer_ty::*;
pub use instruction_gen::InstructionGenQueryGroup;
pub use rust_gen::RustGenQueryGroup;
pub use semantics_entity::ControlEntityVersion;
pub use semantics_entity::EntityQueryGroup;
pub use semantics_package::PackageQueryGroup;
pub use token::TokenQueryGroup;
pub use word::InternWord;

use check_utils::*;
use entity_route::EntityRoutePtr;
use print_utils::*;
use semantics_entity::{EntityKind, EntityVersionControl};
use std::{collections::HashMap, fmt};
use sync_utils::ARwLock;

#[salsa::database(
    file::FileQueryStorage,
    token::TokenQueryGroupStorage,
    entity_route_query::ScopeQueryGroupStorage,
    ast::AstQueryGroupStorage,
    husky_fmt::FormatQueryGroupStorage,
    decl::DeclQueryGroupStorage,
    infer_ty::InferTyQueryGroupStorage,
    infer_contract::InferContractQueryGroupStorage,
    semantics_entity::EntityQueryGroupStorage,
    semantics_package::PackageQueryGroupStorage,
    feature::FeatureQueryGroupStorage,
    diagnostic::DiagnosticQueryStorage,
    instruction_gen::InstructionGenQueryGroupStorage,
    rust_gen::RustGenQueryStorage
)]
pub struct HuskyLangCompileTime {
    storage: salsa::Storage<HuskyLangCompileTime>,
    file_unique_allocator: file::UniqueFileAllocator,
    word_unique_allocator: word::WordAllocator,
    scope_unique_allocator: entity_route::ScopeInterner,
    live_docs: ARwLock<HashMap<file::FilePtr, ARwLock<String>>>,
    vc: semantics_entity::EntityVersionControl,
    features: feature::FeatureUniqueAllocator,
}

pub trait AskCompileTime {
    fn compile_time(&self, version: usize) -> &HuskyLangCompileTime;
}
