use std::sync::Arc;

use fold::FoldStorage;

use crate::{builder::ContractSheetBuilder, *};

#[salsa::query_group(InferContractQueryGroupStorage)]
pub trait InferContractSalsaQueryGroup: InferEntityRouteQueryGroup {
    fn contract_sheet(&self, file: FilePtr) -> EntitySyntaxResultArc<ContractSheet>;
}

pub trait InferContractQueryGroup: InferContractSalsaQueryGroup {}

pub(crate) fn contract_sheet(
    db: &dyn InferContractSalsaQueryGroup,
    file: FilePtr,
) -> EntitySyntaxResultArc<ContractSheet> {
    let mut builder = ContractSheetBuilder::new(db, file)?;
    let ast_text = db.ast_text(file)?;
    builder.infer_all(ast_text.folded_results.iter());
    Ok(Arc::new(builder.finish()))
}
