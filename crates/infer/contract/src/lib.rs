mod builder;
mod query;
mod sheet;

pub use query::*;
pub use sheet::*;

use ast::RawExprIdx;
use check_utils::*;
use entity_syntax::EntitySyntaxResultArc;
use file::FilePtr;
use infer_entity_route::InferEntityRouteQueryGroup;
use infer_error::InferResult;
use print_utils::*;
use vm::{EagerContract, LazyContract};

pub trait InferContract {
    fn contract_sheet(&self) -> &ContractSheet;

    fn lazy_expr_contract(&self, raw_expr_idx: RawExprIdx) -> InferResult<LazyContract> {
        self.contract_sheet().lazy_expr_contract(raw_expr_idx)
    }

    fn eager_expr_contract(&self, raw_expr_idx: RawExprIdx) -> InferResult<EagerContract> {
        self.contract_sheet().eager_expr_contract(raw_expr_idx)
    }
}
