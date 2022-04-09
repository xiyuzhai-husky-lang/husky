mod builder;
mod copy;
mod sheet;

use ast::RawExprIdx;
use check_utils::*;
use copy::is_copyable;
use entity_route::EntityRoutePtr;
use entity_route_query::{EntityRouteQueryGroup, ScopeResultArc};
use file::FilePtr;
use infer_error::InferResult;
use infer_ty::InferTyQueryGroup;
use print_utils::*;
use sheet::*;
use vm::{EagerContract, LazyContract};

#[salsa::query_group(InferContractQueryGroupStorage)]
pub trait InferContractSalsaQueryGroup:
    EntityRouteQueryGroup + ast::AstQueryGroup + InferTyQueryGroup
{
    fn contract_sheet(&self, file: FilePtr) -> ScopeResultArc<ContractSheet>;

    fn is_copyable(&self, ty: EntityRoutePtr) -> bool;
}

pub trait InferContractQueryGroup: InferContractSalsaQueryGroup {
    fn lazy_expr_contract_result(
        &self,
        file: FilePtr,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<LazyContract> {
        let contract_sheet = self.contract_sheet(file)?;
        contract_sheet.lazy_expr_contract_result(raw_expr_idx)
    }

    fn eager_expr_contract_result(
        &self,
        file: FilePtr,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<EagerContract> {
        let contract_sheet = self.contract_sheet(file)?;
        contract_sheet.eager_expr_contract_result(raw_expr_idx)
    }
}
