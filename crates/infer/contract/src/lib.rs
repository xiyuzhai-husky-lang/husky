mod builder;
mod copy;
mod query;
mod sheet;

pub use query::*;
pub use sheet::*;

use ast::RawExprIdx;
use check_utils::*;
use copy::*;
use entity_route::EntityRoutePtr;
use entity_route_query::ScopeResultArc;
use file::FilePtr;
use infer_entity_route::InferTyQueryGroup;
use infer_error::InferResult;
use print_utils::*;
use vm::{EagerContract, LazyContract};

pub trait InferContract {
    fn contract_sheet(&self) -> &ContractSheet;

    fn lazy_expr_contract_result(&self, raw_expr_idx: RawExprIdx) -> InferResult<LazyContract> {
        self.contract_sheet()
            .lazy_expr_contract_result(raw_expr_idx)
    }

    fn eager_expr_contract_result(&self, raw_expr_idx: RawExprIdx) -> InferResult<EagerContract> {
        self.contract_sheet()
            .eager_expr_contract_result(raw_expr_idx)
    }
}
