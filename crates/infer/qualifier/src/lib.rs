mod builder;
mod eager;
mod lazy;
mod query;
mod sheet;

pub use eager::*;
pub use lazy::*;
pub use query::*;
pub use sheet::*;

use entity_route::EntityRoutePtr;
use print_utils::msg_once;

use ast::RawExprIdx;
use infer_error::InferResult;

use text::Row;
use vm::*;
use word::Identifier;

pub trait InferQualifiedType {
    fn qualified_ty_sheet(&self) -> &QualifiedTySheet;

    fn lazy_expr_qualified_ty(&self, raw_expr_idx: RawExprIdx) -> InferResult<EagerQualifiedType> {
        self.qualified_ty_sheet()
            .lazy_expr_qualified_ty(raw_expr_idx)
    }

    fn eager_expr_qualified_ty(&self, raw_expr_idx: RawExprIdx) -> InferResult<EagerQualifiedType> {
        self.qualified_ty_sheet()
            .eager_expr_qualified_ty(raw_expr_idx)
    }

    fn eager_variable_qualified_ty(
        &self,
        varname: Identifier,
        init_row: Row,
    ) -> InferResult<EagerQualifiedType> {
        self.qualified_ty_sheet()
            .eager_variable_qualified_ty(varname, init_row)
    }
}
