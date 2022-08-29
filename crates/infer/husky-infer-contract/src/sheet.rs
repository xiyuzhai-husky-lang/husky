use crate::*;
use arena::map::ArenaMap;
use husky_ast::RawExprMap;
use husky_infer_entity_route::EntityRouteSheet;
use husky_infer_error::*;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq)]
pub struct ContractSheet {
    pub entity_route_sheet: Arc<EntityRouteSheet>,
    pub lazy_expr_contract_results: RawExprMap<InferResult<LazyContract>>,
    pub eager_expr_contract_results: RawExprMap<InferResult<EagerContract>>,
}

impl ContractSheet {
    pub(crate) fn new(entity_route_sheet: Arc<EntityRouteSheet>) -> Self {
        Self {
            lazy_expr_contract_results: ArenaMap::new(&entity_route_sheet.ast_text.arena),
            eager_expr_contract_results: ArenaMap::new(&entity_route_sheet.ast_text.arena),
            entity_route_sheet,
        }
    }

    pub(crate) fn lazy_expr_contract(&self, raw_expr_idx: RawExprIdx) -> InferResult<LazyContract> {
        if let Some(contract_result) = self.lazy_expr_contract_results.get(raw_expr_idx) {
            contract_result.clone()
        } else {
            Err(derived!(format!("contract not inferred")))
        }
    }

    pub(crate) fn eager_expr_contract(
        &self,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<EagerContract> {
        if let Some(contract_result) = self.eager_expr_contract_results.get(raw_expr_idx) {
            contract_result.clone()
        } else {
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
