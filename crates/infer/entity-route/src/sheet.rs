mod expr;
mod var;

use std::{collections::HashMap, sync::Arc};

use ast::{AstText, RawExpr};
use builder::TySheetBuilder;
use fold::FoldStorage;
use infer_decl::MemberIdx;
use text::Row;
use word::CustomIdentifier;

use super::*;

pub(crate) fn entity_route_sheet(
    db: &dyn InferTyQueryGroup,
    file: FilePtr,
) -> ScopeResultArc<EntityRouteSheet> {
    let ast_text = db.ast_text(file)?;
    let mut ty_sheet_builder = TySheetBuilder::new(db, ast_text.clone());
    ty_sheet_builder.infer_all(ast_text.folded_results.fold_iter(0));
    Ok(Arc::new(ty_sheet_builder.finish()))
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EntityRouteSheet {
    pub ast_text: Arc<AstText>,
    pub(crate) expr_tys: HashMap<RawExprIdx, InferResult<EntityRoutePtr>>,
    pub(crate) call_routes: HashMap<RawExprIdx, InferResult<EntityRoutePtr>>,
    pub(crate) member_indices: HashMap<RawExprIdx, InferResult<MemberIdx>>,
    pub(crate) variable_tys: HashMap<(CustomIdentifier, Row), Option<EntityRoutePtr>>,
}

impl EntityRouteSheet {
    pub(crate) fn new(ast_text: Arc<AstText>) -> Self {
        Self {
            expr_tys: Default::default(),
            variable_tys: Default::default(),
            call_routes: Default::default(),
            member_indices: Default::default(),
            ast_text,
        }
    }

    pub fn expr_ty_result(&self, expr_idx: RawExprIdx) -> InferResult<EntityRoutePtr> {
        match &self.expr_tys[&expr_idx] {
            Ok(ty) => Ok(*ty),
            Err(e) => Err(e.derived()),
        }
    }

    pub fn call_route_result(&self, expr_idx: RawExprIdx) -> InferResult<EntityRoutePtr> {
        match &self.call_routes[&expr_idx] {
            Ok(call_route) => Ok(*call_route),
            Err(e) => Err(e.derived()),
        }
    }

    pub fn member_idx_result(&self, expr_idx: RawExprIdx) -> InferResult<MemberIdx> {
        match &self.member_indices[&expr_idx] {
            Ok(member_idx) => Ok(*member_idx),
            Err(e) => Err(e.derived()),
        }
    }

    pub fn errors(&self) -> Vec<&InferError> {
        let mut errors: Vec<&InferError> = Vec::new();
        for (_, result) in self.expr_tys.iter() {
            match result {
                Ok(_) => (),
                Err(error) => match error.variant {
                    InferErrorVariant::Derived => (),
                    InferErrorVariant::Original { .. } => errors.push(error),
                },
            }
        }
        errors
    }
}
