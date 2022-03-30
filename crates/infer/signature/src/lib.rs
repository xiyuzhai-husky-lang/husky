mod call;
mod feature;
mod implicit;
mod ty;

pub use call::*;
use file::FilePtr;
pub use ty::*;

use ast::*;
use feature::*;
use fold::FoldStorage;
use implicit::*;
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
    fn package_input_ty(&self, main_file: FilePtr) -> InferResult<ScopePtr>;
    fn package_output_ty(&self, main_file: FilePtr) -> InferResult<ScopePtr>;
}
