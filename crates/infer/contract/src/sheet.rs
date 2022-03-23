use std::{collections::HashMap, sync::Arc};

use fold::FoldStorage;
use infer_ty::TySheet;

use crate::{builder::ContractSheetBuilder, *};

#[derive(Debug, PartialEq, Eq)]
pub struct ContractSheet {
    pub(crate) ty_sheet: Arc<TySheet>,
    pub(crate) lazy_expr_contract_results: HashMap<RawExprIdx, InferResult<LazyContract>>,
    pub(crate) eager_expr_contract_results: HashMap<RawExprIdx, InferResult<EagerContract>>,
}

impl ContractSheet {
    pub(crate) fn new(ty_sheet: Arc<TySheet>) -> Self {
        Self {
            ty_sheet,
            lazy_expr_contract_results: Default::default(),
            eager_expr_contract_results: Default::default(),
        }
    }

    pub(crate) fn lazy_expr_contract_result(
        &self,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<LazyContract> {
        self.lazy_expr_contract_results[&raw_expr_idx].clone()
    }

    pub(crate) fn eager_expr_contract_result(
        &self,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<EagerContract> {
        self.eager_expr_contract_results[&raw_expr_idx].clone()
    }
}

pub(crate) fn contract_sheet(
    db: &dyn InferContractSalsaQueryGroup,
    file: FilePtr,
) -> ScopeResultArc<ContractSheet> {
    let ty_sheet = db.ty_sheet(file)?;
    let mut builder = ContractSheetBuilder::new(db, file, ty_sheet);
    let ast_text = db.ast_text(file)?;
    builder.infer_all(ast_text.folded_results.fold_iter(0));
    Ok(Arc::new(builder.finish()))
}
