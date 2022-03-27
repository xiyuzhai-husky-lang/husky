mod call;
mod feature;
mod ty;

pub use call::*;
pub use ty::*;

use ast::*;

use feature::*;
use fold::FoldStorage;
use infer_error::*;
use scope::*;
use scope_query::*;
use std::sync::Arc;
use syntax_types::RawTyKind;
use vm::Compiled;
use word::CustomIdentifier;

#[salsa::query_group(InferSignatureQueryGroupStorage)]
pub trait InferSignatureQueryGroup: ScopeQueryGroup + ast::AstQueryGroup {
    fn call_signature(&self, scope: ScopePtr) -> InferResultArc<CallSignature>;
    fn ty_signature(&self, scope: ScopePtr) -> InferResultArc<TySignature>;
    fn feature_signature(&self, scope: ScopePtr) -> InferResultArc<FeatureSignature>;
}
