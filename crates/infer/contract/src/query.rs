use crate::*;

#[salsa::query_group(InferContractQueryGroupStorage)]
pub trait InferContractSalsaQueryGroup: InferTyQueryGroup {
    fn contract_sheet(&self, file: FilePtr) -> ScopeResultArc<ContractSheet>;
}

pub trait InferContractQueryGroup: InferContractSalsaQueryGroup {}
