use crate::*;
use husky_expr_ty::ExprTypeDb;
use husky_hir_ty::db::HirTypeDb;

pub trait HirEagerExprDb: salsa::DbWithJar<HirEagerExprJar> + ExprTypeDb + HirTypeDb {}

impl<Db> HirEagerExprDb for Db where Db: salsa::DbWithJar<HirEagerExprJar> + ExprTypeDb + HirTypeDb {}

#[salsa::jar(db = HirEagerExprDb)]
pub struct HirEagerExprJar(HirEagerExprRegion);
