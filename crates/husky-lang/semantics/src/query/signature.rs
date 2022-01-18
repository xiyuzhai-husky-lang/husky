use std::sync::Arc;

use scope::{CallSignature, ScopeId};
use scope_query::ScopeQueryGroup;

use crate::{error::*, SemanticResult};

#[salsa::query_group(CallSignatureQueryGroupStorage)]
pub trait CallSignatureQueryGroup: ScopeQueryGroup {
    fn call_signature(&self, scope: ScopeId) -> SemanticResult<Arc<CallSignature>>;
}

fn call_signature(
    this: &dyn CallSignatureQueryGroup,
    scope: ScopeId,
) -> SemanticResult<Arc<CallSignature>> {
    let source = this.scope_source(scope)?;
    match source {
        scope::ScopeSource::Builtin(data) => {
            Ok(Arc::new(not_none!(data.call_signature.as_ref()).clone()))
        }
        scope::ScopeSource::WithinBuiltinModule => todo!(),
        scope::ScopeSource::WithinModule {
            file_id,
            token_group_index,
        } => todo!(),
        scope::ScopeSource::Module { file_id } => todo!(),
    }
}
