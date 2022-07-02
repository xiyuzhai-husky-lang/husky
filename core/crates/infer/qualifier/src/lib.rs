mod builder;
mod eager;
mod lazy;
mod query;
mod sheet;

pub use eager::*;
pub use lazy::*;
pub use query::*;
pub use sheet::*;

use husky_ast::RawExprIdx;
use husky_entity_route::EntityRoutePtr;
use husky_liason_semantics::*;
use husky_text::{Row, TextRange};
use infer_contract::*;
use infer_error::InferResult;
use print_utils::emsg_once;
use vm::*;
use word::{CustomIdentifier, Identifier};

pub trait InferQualifiedTy {
    fn qualified_ty_sheet(&self) -> &QualifiedTySheet;

    fn lazy_expr_qualified_ty(
        &self,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<LazyValueQualifiedTy> {
        self.qualified_ty_sheet()
            .lazy_expr_qualified_ty(raw_expr_idx)
    }

    fn eager_expr_qualified_ty(
        &self,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<EagerValueQualifiedTy> {
        self.qualified_ty_sheet()
            .eager_expr_qualified_ty(raw_expr_idx)
    }

    fn eager_variable_qualified_ty(
        &self,
        varname: CustomIdentifier,
        init_range: TextRange,
    ) -> InferResult<EagerVariableQualifiedTy> {
        self.qualified_ty_sheet()
            .eager_variable_qualified_ty(varname, init_range)
    }

    fn lazy_variable_qualified_ty(
        &self,
        varname: CustomIdentifier,
        init_range: TextRange,
    ) -> InferResult<LazyVariableQualifiedTy> {
        self.qualified_ty_sheet()
            .lazy_variable_qualified_ty(varname, init_range)
    }
}
