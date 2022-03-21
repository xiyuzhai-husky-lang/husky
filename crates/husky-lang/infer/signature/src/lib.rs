mod call;
mod contract;
mod ty;

pub use call::*;
pub use ty::*;

use ast::*;
use common::*;
use fold::FoldStorage;
use infer_error::*;
use scope::*;
use scope_query::*;
use std::sync::Arc;
use syntax_types::{ListOpr, Opr, RawTyKind};
use vm::{Compiled, EnumLiteralValue};
use word::{BuiltinIdentifier, CustomIdentifier, ImplicitIdentifier};

#[salsa::query_group(InferSignatureQueryGroupStorage)]
pub trait InferSignatureQueryGroup: ScopeQueryGroup + ast::AstQueryGroup {
    fn call_signature(&self, scope: ScopePtr) -> InferResultArc<CallSignature>;
    fn ty_signature(&self, scope: ScopePtr) -> InferResultArc<TySignature>;
}
