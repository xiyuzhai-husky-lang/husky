use crate::*;

#[salsa::query_group(InferContractQueryGroupStorage)]
pub trait InferContractSalsaQueryGroup: InferTyQueryGroup {
    fn contract_sheet(&self, file: FilePtr) -> ScopeResultArc<ContractSheet>;

    fn is_copy_constructible(&self, ty: EntityRoutePtr) -> bool;
}

pub trait InferContractQueryGroup: InferContractSalsaQueryGroup {}
