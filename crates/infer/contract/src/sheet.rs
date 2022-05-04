use std::{collections::HashMap, sync::Arc};

use fold::FoldStorage;
use infer_entity_route::EntityRouteSheet;
use infer_error::*;

use crate::{builder::ContractSheetBuilder, *};

#[derive(Debug, PartialEq, Eq)]
pub struct ContractSheet {
    pub entity_route_sheet: Arc<EntityRouteSheet>,
    pub(crate) lazy_expr_contract_results: HashMap<RawExprIdx, InferResult<LazyContract>>,
    pub(crate) eager_expr_contract_results: HashMap<RawExprIdx, InferResult<EagerContract>>,
}

impl ContractSheet {
    pub(crate) fn new(ty_sheet: Arc<EntityRouteSheet>) -> Self {
        Self {
            entity_route_sheet: ty_sheet,
            lazy_expr_contract_results: Default::default(),
            eager_expr_contract_results: Default::default(),
        }
    }

    pub(crate) fn lazy_expr_contract(&self, raw_expr_idx: RawExprIdx) -> InferResult<LazyContract> {
        if let Some(contract_result) = self.lazy_expr_contract_results.get(&raw_expr_idx) {
            contract_result.clone()
        } else {
            p!(self.entity_route_sheet.ast_text.arena[raw_expr_idx]);
            Err(derived!(format!("contract not inferred")))
        }
    }

    pub(crate) fn eager_expr_contract(
        &self,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<EagerContract> {
        if let Some(contract_result) = self.eager_expr_contract_results.get(&raw_expr_idx) {
            contract_result.clone()
        } else {
            p!(self.entity_route_sheet.ast_text.arena[raw_expr_idx]);
            Err(derived!(format!("contract not inferred")))
        }
    }

    pub fn errors(&self) -> Vec<&InferError> {
        let mut errors: Vec<&InferError> = Vec::new();
        for (_, result) in self.lazy_expr_contract_results.iter() {
            match result {
                Ok(_) => (),
                Err(error) => match error.variant {
                    InferErrorVariant::Derived { .. } => (),
                    InferErrorVariant::Original { .. } => errors.push(error),
                },
            }
        }
        for (_, result) in self.eager_expr_contract_results.iter() {
            match result {
                Ok(_) => (),
                Err(error) => match error.variant {
                    InferErrorVariant::Derived { .. } => (),
                    InferErrorVariant::Original { .. } => errors.push(error),
                },
            }
        }
        errors
    }
}
