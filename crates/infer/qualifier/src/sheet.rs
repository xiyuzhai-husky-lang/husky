use std::collections::HashMap;

use text::Row;
use word::CustomIdentifier;

use crate::*;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct QualifiedTySheet {
    pub(crate) variable_qualified_tys: HashMap<(CustomIdentifier, Row), InferResult<QualifiedTy>>,
    pub(crate) qualified_tys: HashMap<RawExprIdx, InferResult<QualifiedTy>>,
}

impl QualifiedTySheet {
    pub fn lazy_expr_qualifier_result(&self, raw_expr_idx: RawExprIdx) -> InferResult<Qualifier> {
        todo!()
    }

    pub fn eager_expr_qualifier_result(&self, raw_expr_idx: RawExprIdx) -> InferResult<Qualifier> {
        todo!()
    }
}
