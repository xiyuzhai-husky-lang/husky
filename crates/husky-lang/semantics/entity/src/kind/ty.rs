use std::sync::Arc;

use semantics_infer::TySignature;
use syntax_types::TyKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ty {
    pub kind: TyKind,
    pub signature: Arc<TySignature>,
}
