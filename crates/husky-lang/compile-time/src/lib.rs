mod impl_load;
mod impl_necessary;
#[cfg(test)]
mod tests;

pub use ast::{AstQueryGroup, AstSalsaQueryGroup};
pub use diagnostic::DiagnosticQuery;
pub use file::{AllocateUniqueFile, FileQueryGroup, LiveFiles};
pub use husky_fmt::FmtQuery;
pub use scope::{AllocateUniqueScope, Scope};
pub use scope_query::{ScopeQueryGroup, ScopeSalsaQueryGroup};
pub use semantics_entity::ControlEntityVersion;
pub use semantics_entity::EntityQueryGroup;
pub use semantics_feature::{AllocateUniqueFeature, FeatureQueryGroup, FeatureQueryGroupStorage};
pub use semantics_package::PackageQueryGroup;
pub use syntax_infer::InferQueryGroup;
pub use token::TokenQueryGroup;
pub use word::InternWord;

use common::*;
use scope::ScopePtr;
use semantics_entity::{EntityKind, EntityVersionControl};
use std::fmt;
use stdx::sync::ARwLock;

#[salsa::database(
    file::FileQueryStorage,
    token::TokenQueryGroupStorage,
    scope_query::ScopeQueryGroupStorage,
    ast::AstQueryGroupStorage,
    husky_fmt::FormatQueryGroupStorage,
    syntax_infer::InferSalsaQueryGroupStorage,
    semantics_entity::EntityQueryGroupStorage,
    semantics_package::PackageQueryGroupStorage,
    semantics_feature::FeatureQueryGroupStorage,
    diagnostic::DiagnosticQueryStorage,
    compiler::CompilerStorage
)]
pub struct HuskyLangCompileTime {
    storage: salsa::Storage<HuskyLangCompileTime>,
    file_unique_allocator: file::UniqueFileAllocator,
    word_unique_allocator: word::WordInterner,
    scope_unique_allocator: scope::UniqueScopeAllocator,
    live_docs: ARwLock<HashMap<file::FilePtr, ARwLock<String>>>,
    vc: semantics_entity::EntityVersionControl,
    features: semantics_feature::FeatureUniqueAllocator,
}
