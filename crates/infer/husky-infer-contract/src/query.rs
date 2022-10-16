use std::sync::Arc;

use fold::FoldableStorage;

use crate::{builder::ContractSheetBuilder, *};

#[salsa::query_group(InferContractQueryGroupStorage)]
pub trait InferContractSalsaQueryGroup: InferEntityRouteQueryGroup {
    fn contract_sheet(&self, file: FileItd) -> EntitySyntaxResultArc<ContractSheet>;
}

pub trait InferContractQueryGroup: InferContractSalsaQueryGroup {}

pub(crate) fn contract_sheet(
    db: &dyn InferContractSalsaQueryGroup,
    file: FileItd,
) -> EntitySyntaxResultArc<ContractSheet> {
    let ast_text = db.ast_text(file)?;
    let mut builder = ContractSheetBuilder::new(db, &ast_text.arena, file)?;
    builder.infer_all(ast_text.folded_results.iter());
    Ok(Arc::new(builder.finish()))
}
