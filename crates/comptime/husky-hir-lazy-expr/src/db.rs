use crate::*;
use husky_expr_ty::ExprTypeDb;

pub trait HirLazyExprDb: salsa::DbWithJar<HirLazyExprJar> + ExprTypeDb {}

impl<Db> HirLazyExprDb for Db where Db: salsa::DbWithJar<HirLazyExprJar> + ExprTypeDb {}

#[salsa::jar(db = HirLazyExprDb)]
pub struct HirLazyExprJar(HirLazyExprRegion);
