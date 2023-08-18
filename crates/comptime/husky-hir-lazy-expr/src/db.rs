use crate::*;
use husky_expr_ty::ExprTypeDb;
use husky_hir_ty::db::HirTypeDb;

pub trait HirLazyExprDb: salsa::DbWithJar<HirLazyExprJar> + ExprTypeDb + HirTypeDb {}

impl<Db> HirLazyExprDb for Db where Db: salsa::DbWithJar<HirLazyExprJar> + ExprTypeDb + HirTypeDb {}

#[salsa::jar(db = HirLazyExprDb)]
pub struct HirLazyExprJar(HirLazyExprRegion);
