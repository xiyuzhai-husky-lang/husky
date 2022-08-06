mod expr;
mod var;

use std::{collections::HashMap, sync::Arc};

use arena::map::ArenaMap;
use builder::EntityRouteSheetBuilder;
use fold::FoldableStorage;
use husky_ast::{AstText, RawExpr};
use husky_dev_utils::dev_src;
use husky_text::{Row, TextRange};
use husky_word::CustomIdentifier;
use infer_decl::MemberIdx;

use super::*;

pub(crate) fn entity_route_sheet(
    db: &dyn InferEntityRouteQueryGroup,
    file: FilePtr,
) -> EntitySyntaxResultArc<EntityRouteSheet> {
    let ast_text = db.ast_text(file)?;
    let mut ty_sheet_builder = EntityRouteSheetBuilder::new(db, &ast_text.arena, ast_text.clone());
    ty_sheet_builder.infer_all(ast_text.folded_results.iter());
    Ok(Arc::new(ty_sheet_builder.finish()))
}

#[derive(Debug, PartialEq, Eq)]
pub struct EntityRouteSheet {
    pub crate_entrance: FilePtr,
    pub ast_text: Arc<AstText>,
    pub(crate) expr_tys: RawExprMap<InferResult<EntityRoutePtr>>,
    pub(crate) function_call_routes: RawExprMap<InferResult<EntityRoutePtr>>, // keys are function idx
    pub(crate) method_call_routes: RawExprMap<InferResult<EntityRoutePtr>>,   // keys are this idx
    pub(crate) variable_tys: HashMap<(CustomIdentifier, TextRange), EntityRoutePtr>,
    pub(crate) extra_errors: Vec<InferError>,
}

impl EntityRouteSheet {
    fn expr(&self, idx: RawExprIdx) -> &RawExpr {
        &self.ast_text.arena[idx]
    }

    pub(crate) fn new(
        crate_entrance: FilePtr,
        ast_text: Arc<AstText>,
        extra_errors: Vec<InferError>,
    ) -> Self {
        Self {
            expr_tys: ArenaMap::new(&ast_text.arena),
            function_call_routes: ArenaMap::new(&ast_text.arena),
            method_call_routes: ArenaMap::new(&ast_text.arena),
            variable_tys: Default::default(),
            ast_text,
            extra_errors,
            crate_entrance: todo!(),
        }
    }

    pub fn expr_ty_result(&self, idx: RawExprIdx) -> InferResult<EntityRoutePtr> {
        if let Some(ref expr_ty) = self.expr_tys.get(idx) {
            match expr_ty {
                Ok(ty) => Ok(*ty),
                Err(e) => Err(e.derived()),
            }
        } else {
            Err(InferError {
                variant: InferErrorVariant::Derived {
                    message: "failed to infer expr ty".into(),
                },
                dev_src: dev_src!(),
            })
        }
    }

    pub fn opt_function_call_route(
        &self,
        function_idx: RawExprIdx,
    ) -> Option<InferResult<EntityRoutePtr>> {
        Some(match self.function_call_routes.get(function_idx)? {
            Ok(call_route) => Ok(*call_route),
            Err(e) => Err(e.derived()),
        })
    }

    pub fn opt_method_call_route(
        &self,
        this_idx: RawExprIdx,
    ) -> Option<InferResult<EntityRoutePtr>> {
        Some(match self.method_call_routes.get(this_idx)? {
            Ok(call_route) => Ok(*call_route),
            Err(e) => Err(e.derived()),
        })
    }

    pub fn original_errors(&self) -> Vec<&InferError> {
        let mut errors: Vec<&InferError> = self
            .extra_errors
            .iter()
            .filter_map(|e| match e.variant {
                InferErrorVariant::Derived { .. } => None,
                InferErrorVariant::Original { .. } => Some(e),
            })
            .collect();
        for result in self.expr_tys.values() {
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
