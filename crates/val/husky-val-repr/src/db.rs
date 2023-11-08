use crate::*;
use husky_hir_lazy_expr::db::HirLazyExprDb;
use husky_val::ValDb;

pub trait ValReprDb: salsa::DbWithJar<ValReprJar> + ValDb + HirLazyExprDb {}

impl<Db> ValReprDb for Db where Db: salsa::DbWithJar<ValReprJar> + ValDb + HirLazyExprDb {}

#[salsa::jar(db = ValReprDb)]
pub struct ValReprJar(ValRepr, val_repr_val, val_repr_expansion);
