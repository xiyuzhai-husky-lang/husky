use std::collections::HashMap;

use infer_error::derived_not_none;
use print_utils::p;
use text::Row;
use word::Identifier;

use crate::*;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct QualifiedTypeSheet {
    pub(crate) eager_variable_qualified_tys:
        HashMap<(Identifier, Row), InferResult<EagerQualifiedType>>,
    pub(crate) lazy_variable_qualified_tys:
        HashMap<(Identifier, Row), InferResult<LazyQualifiedType>>,
    pub(crate) eager_expr_qualified_tys: HashMap<RawExprIdx, InferResult<EagerQualifiedType>>,
    pub(crate) lazy_expr_qualified_tys: HashMap<RawExprIdx, InferResult<LazyQualifiedType>>,
}

impl QualifiedTypeSheet {
    pub fn lazy_expr_qualified_ty(
        &self,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<EagerQualifiedType> {
        todo!()
    }

    pub fn eager_expr_qualified_ty(
        &self,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<EagerQualifiedType> {
        match derived_not_none!(self.eager_expr_qualified_tys.get(&raw_expr_idx))? {
            Ok(qt) => Ok(*qt),
            Err(e) => Err(e.derived()),
        }
    }

    pub fn eager_variable_qualified_ty(
        &self,
        varname: Identifier,
        init_row: Row,
    ) -> InferResult<EagerQualifiedType> {
        match derived_not_none!(self.eager_variable_qualified_tys.get(&(varname, init_row)))? {
            Ok(qt) => Ok(*qt),
            Err(e) => Err(e.derived()),
        }
    }
}
