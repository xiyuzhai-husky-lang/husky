use std::sync::Arc;

use common::Upcast;
use scope::{FuncSignature, ScopeId};
use scope_query::ScopeQueryGroup;

use crate::{error::*, SemanticResult};

#[salsa::query_group(InferQueryGroupStorage)]
pub trait InferQueryGroup: ScopeQueryGroup {
    fn func_signature(&self, scope: ScopeId) -> SemanticResult<Arc<FuncSignature>>;
}

fn func_signature(
    this: &dyn InferQueryGroup,
    scope: ScopeId,
) -> SemanticResult<Arc<FuncSignature>> {
    let source = this.scope_source(scope)?;
    match source {
        scope::ScopeSource::Builtin(data) => Ok(Arc::new(match &data.signature {
            scope::ScopeSignature::Func(signature) => signature.clone(),
            _ => panic!(),
        })),
        scope::ScopeSource::WithinBuiltinModule => todo!(),
        scope::ScopeSource::WithinModule {
            file_id,
            token_group_index,
        } => todo!(),
        scope::ScopeSource::Module { file_id } => todo!(),
    }
}
