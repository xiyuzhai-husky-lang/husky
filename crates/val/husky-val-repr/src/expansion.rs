use crate::db::ValReprDb;
use crate::*;
use husky_hir_lazy_expr::{HirLazyExprMap, HirLazyStmtMap};
use husky_val::ValOpr;

#[derive(Debug, PartialEq, Eq)]
pub struct ValReprExpansion {
    expr_val_repr_map: HirLazyExprMap<ValRepr>,
    stmt_val_repr_map: HirLazyStmtMap<ValRepr>,
}

#[salsa::tracked(jar = ValReprJar, return_ref)]
pub(crate) fn val_repr_expansion(
    db: &dyn ValReprDb,
    val_repr: ValRepr,
) -> Option<ValReprExpansion> {
    match val_repr.opr(db) {
        ValOpr::Fugitive(_) => todo!(),
        ValOpr::Require => None,
    }
}

pub struct ValReprExpansionBuilder<'a> {
    db: &'a dyn ValReprDb,
    expr_val_repr_map: HirLazyExprMap<ValRepr>,
    stmt_val_repr_map: HirLazyStmtMap<ValRepr>,
}

impl<'a> ValReprExpansionBuilder<'a> {
    fn new(db: &'a dyn ValReprDb) -> Self {
        todo!()
    }
}
